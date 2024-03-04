use clap::Parser;
use futures::{stream::FuturesUnordered, StreamExt as _};
use reqwest::Url;
use std::path::PathBuf;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    runtime::Builder,
};

/// util to download web pages
#[derive(Parser)]
#[command(version, about, long_about = None, author)]
struct Args {
    /// The file with urls, newline separated
    file: PathBuf,

    /// maximum thread usage
    #[arg(long)]
    max_threads: Option<usize>,
}

/// download and save the site
async fn download(link: Url) -> Result<(), String> {
    let fname = link.as_ref().replace('/', "_");
    let fname: PathBuf = format!("./\"{}\".html", fname).parse().unwrap();

    let res = reqwest::get(link)
        .await
        .map_err(|_| "cannot get".to_string())?;

    let bytes = res
        .bytes()
        .await
        .map_err(|_| "cannot get bytes".to_string())?;

    let mut file = File::create(&fname)
        .await
        .map_err(|e| format!("cannot create file {e:?}"))?;

    match file.write_all(&bytes).await {
        Ok(_) => Ok(()),
        Err(_) => {
            drop(file);
            tokio::fs::remove_file(&fname).await.unwrap();
            Err("cannot write to file".to_string())
        }
    }
}

fn main() {
    let args = Args::parse();

    let cpus = num_cpus::get();

    let threads = args.max_threads.unwrap_or(cpus);

    let runtime = Builder::new_multi_thread()
        .worker_threads(threads)
        .thread_name("my-example")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let file = File::open(args.file).await.unwrap();

        let reader = BufReader::new(file);

        let lines = tokio_stream::wrappers::LinesStream::new(reader.lines());

        lines
            .fold(FuturesUnordered::new(),|futs,l| async {
                if let Ok(line) = l {
                    if let Ok(url) =  Url::parse(&line) {
                        let handle  = tokio::spawn(download(url));
                        futs.push(handle);
                    };
                };
                futs
            })
            .await
            .collect::<Vec<_>>()
            .await
    });
}
