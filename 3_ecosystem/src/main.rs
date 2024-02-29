use std::{io::BufRead, path::{Path, PathBuf}};
use crossbeam_channel::{unbounded, Receiver};
use clap::Parser;

/// Strips the JPEG metadata and minimizes size
#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Args {

    //dir for output files
    #[arg(short,long)]
    out_dir: Option<PathBuf>,

    /// how many images are processed at the same time
    #[arg(short, long)]
    jobs: Option<usize>,

    /// the % of original quality, default is 100
    #[arg(short, long)]
    quality: Option<f64>,

    /// path to a file with sources
    #[arg(short, long)]
    images_file: Option<PathBuf>,

    #[arg(long)]
    config_file: Option<PathBuf>,

    images: Option<Vec<String>>

}

#[derive(serde::Deserialize,Debug)]
struct Conf {
    out_dir: PathBuf,
    jobs: usize,
    quality: f64,

    image_list: Option<PathBuf>
}

fn main() {
    let args = Args::parse();

    let mut builder = config::Config::builder()
        .add_source(
            config::File::new(
                &args.config_file.clone().unwrap_or("./config.json".into()).to_string_lossy(),
                 config::FileFormat::Json
            )
        )
        .add_source(
            config::Environment::with_convert_case(
                config::Case::UpperSnake
            )
        )
        .set_default(
            "out_dir", 
            args.out_dir.clone().unwrap_or("./".into()).display().to_string()
        )
        .unwrap()
        .set_default(
            "jobs", 
            (num_cpus::get() / 2).to_string()
        )
        .unwrap()
        .set_default(
            "quality", 
            "100.0"
        )
        .unwrap();

    if let Some(pathb) = args.images_file {
        builder = builder.set_default(
            "image_list",
            pathb.display().to_string()
        )
        .unwrap();
    }

    let conf: Conf = builder.build().unwrap().try_deserialize().unwrap();

    work(&conf,&args.images.unwrap_or(vec![]));
}


fn process_file(path: &Path,cfg: &Conf) -> Result<(),String> {
    unimplemented!()
}

fn process_url(path: &url::Url,cfg: &Conf) -> Result<(),String> {
    unimplemented!()
}

fn worker(rcv: Receiver<String>,cfg: &Conf) {
    while let Ok(s) = rcv.recv() {
        if let Ok(url) = s.parse::<url::Url>() {
            process_url(&url,cfg);
        }
        if let Ok(path) = s.parse::<PathBuf>() {
            process_file(&path, cfg);
        }
    }
}


fn work(conf: &Conf,cli_req: &[String]) {
    let (snd, rcv) = unbounded::<String>();

    for i in cli_req.iter() {
        let _ =snd.send(i.clone());
    }

    // closure that read lines from stdin
    let mut consume_stdin = {
        let snd = snd.clone();
        let mut stdin = std::io::BufReader::new(std::io::stdin());
        move || {
            let mut line = String::new();
            
            loop {
                let _ = stdin.read_line(&mut line);
    
                match &*line {
                    "q!\n" => {
                        break;
                    },
                    line => {
                        let _ = snd.send(line.trim().into());
                    }
                }
                line.clear();
            }
        }
    };


    //read file and fill the channel with its content
    {
        let snd = snd.clone();
        if let Some(fname) = &conf.image_list {
            if let Ok(file) = std::fs::File::open(fname) {
                let mut reader = std::io::BufReader::new(file);
                let mut line = String::new();
                
                loop {
                    match reader.read_line(&mut line) {
                        Ok(0) => break,
                        Ok(_) => {
                            let _ = snd.send(line.trim().into());
                            line.clear()
                        }
                        Err(_) => break,
                    }
                }
    
            }
        }
    }

    std::thread::scope({
        let rcv = rcv.clone();
        move |s| {
            for _ in 0..conf.jobs {
                let rcv = rcv.clone();
                s.spawn(move || worker(rcv.clone(),conf));
            };
            consume_stdin();
        }
    });

}