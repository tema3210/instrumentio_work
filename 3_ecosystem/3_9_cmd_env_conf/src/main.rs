use clap::Parser;
use config::{Config, Environment, File, FileFormat};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Opts {
    /// the config file
    #[clap(long, short)]
    conf: Option<String>,

    /// Enables debug mode
    #[clap(short, long)]
    debug: bool,
}

#[derive(serde::Deserialize)]
struct Conf {
    conf_file: String,
    name: String,
}

fn main() {
    let args = Opts::parse();

    let builder = Config::builder()
        .add_source(File::new("config.toml", FileFormat::Toml))
        .add_source(Environment::with_convert_case(config::Case::UpperSnake).prefix("CONF_"))
        .set_default("conf_file", args.conf.as_deref().unwrap_or("config.toml"))
        .unwrap()
        .set_default("name", "Joe Biden")
        .unwrap();

    let conf: Conf = builder.build().unwrap().try_deserialize().unwrap();

    println!("Hello {}! the config is: {}", &conf.name, &conf.conf_file);
}
