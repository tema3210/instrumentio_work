use std::{fmt::format, path::PathBuf};
use clap::Parser;
use reqwest::Url;
use tokio::{fs::File, io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, runtime::Builder};

/// util to download web pages
#[derive(Parser)]
#[command(version, about, long_about = None, author)]
struct Args {
    /// maximum thread usage
    max_threads: Option<usize>,
    
    file: PathBuf
}

/// download and save the site
async fn download(link: Url) -> Option<()> {
    let fname: PathBuf = format!("./{}",link.as_ref()).parse().unwrap();

    let res = reqwest::get(link).await.ok()?;

    let bytes = res.bytes().await.ok()?;

    let mut file = File::create(&fname).await.ok()?;

    match file.write_all(&bytes).await {
        Ok(_) => return Some(()),
        Err(_) => {
            drop(file);
            tokio::fs::remove_file(&fname).await.unwrap();
            None
        },
    }
}


fn main() {
    let args = Args::parse();

    let cpus = num_cpus::get();

    let threads = args.max_threads.unwrap_or(cpus );


    let runtime = Builder::new_multi_thread()
        .worker_threads(threads)
        .thread_name("my-example")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();

    runtime.block_on(async {
        let file = File::open(args.file).await.unwrap();

        let mut reader = BufReader::new(file);

        let mut line = String::new();

        let mut handles = vec![];
        while let Ok(l) = AsyncBufReadExt::read_line(&mut reader,&mut line).await {
            if l == 0 { break; }

            let line = line.clone();

            if let Ok(url) = Url::parse(&line) {
                let h = tokio::spawn(download(url.clone()));
                handles.push((h,url));
            }
        }

        // Is there any primitive for this? like Go's groups
        for (h,url) in handles {
            match h.await {
                Ok(Some(())) => {
                    println!("Loaded {url}");
                },
                Ok(None) => {
                    println!("Cannot load {url}")
                }
                Err(_) => {
                    println!("Internal error")
                },
            }
        }

    });
}
