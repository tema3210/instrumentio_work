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


type Compression = u8;

// into usize percents of quality
fn process_quality(input: f64) -> Compression {
    let sane  = (input % 100.0).clamp(10.0,90.0).round() as Compression;
    100 - sane // complementary
    // sane // direct
}

fn process_image_bytes<B: AsRef<[u8]>>(bytes: B, compress: Compression) -> Result<Vec<u8>,String> {
    // Decode the image
    let img = image::load_from_memory(bytes.as_ref())
        .map_err(|err| format!("Failed to decode image: {}", err))?;

    // Create an RGBA image to strip metadata
    let rgba_image: image::RgbaImage = img.to_rgba8();

    // Encode the image with compression
    let mut compressed_data = std::io::Cursor::new(Vec::new());
    rgba_image.write_to(&mut compressed_data, image::ImageOutputFormat::Jpeg(compress))
        .map_err(|err| format!("Failed to compress image: {}", err))?;

    Ok(compressed_data.into_inner())
}

fn process_file(path: &Path,cfg: &Conf) -> Result<(),String> {
    let Some(fname) = path.file_name() else {
        return Err(format!("supplied path is a dir: {}",path.display()))
    };

    let image_data = std::fs::read(path).map_err(|_| format!("cannot read file: {}",path.display()))?;
    
    let image_data = process_image_bytes(image_data, process_quality(cfg.quality))?;

    save_bytes(image_data,cfg,fname)
}

fn process_url(url: &url::Url,cfg: &Conf) -> Result<(),String> {
    // Fetch the image from the URL
    let response = reqwest::blocking::get(url.as_str())
        .map_err(|err| format!("Failed to fetch image from URL: {}", err))?;

    // Ensure the request was successful (status code 2xx)
    if !response.status().is_success() {
        return Err(format!("Failed to fetch image. Status code: {}", response.status()));
    }

    let file_name = url.path_segments().and_then(|segments| segments.last()).unwrap_or(url.as_str());

    // Read the image data
    let image_data = response.bytes()
        .map_err(|err| format!("Failed to read image data: {}", err))?;

    let image_data = process_image_bytes(image_data, process_quality(cfg.quality))?;

    save_bytes(image_data,cfg,file_name)
    
}

fn save_bytes<B: AsRef<[u8]>>(buff: B, cfg: &Conf, fname: impl AsRef<Path>) -> Result<(),String> {
    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(&cfg.out_dir)
        .map_err(|err| format!("Failed to create output directory: {}", err))?;

    // Construct the output file path
    let file_path = cfg.out_dir.join(fname.as_ref());

    // Save the image data to the file
    std::fs::write(file_path, buff.as_ref())
        .map_err(|err| format!("Failed to save image to file: {}", err))?;

    Ok(())
}

fn worker(rcv: Receiver<String>,cfg: &Conf) {
    while let Ok(s) = rcv.recv() {
        if let Ok(url) = s.parse::<url::Url>() {
            let _ = process_url(&url,cfg);
        }
        if let Ok(path) = s.parse::<PathBuf>() {
            let _ = process_file(&path, cfg);
        }
    }
}

fn work(conf: &Conf,cli_req: &[String]) {
    let (snd, rcv) = unbounded::<String>();

    // populate cli req to queue
    for i in cli_req.iter() {
        let _ = snd.send(i.clone());
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