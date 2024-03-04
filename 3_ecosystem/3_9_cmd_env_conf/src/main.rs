#![feature(adt_const_params)]
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

mod conf {
    /// if only they had literal support... see https://github.com/serde-rs/serde/issues/368

    const fn default_bool<const V: bool>() -> bool {
        V
    }
    const fn default_usize<const V: usize>() -> usize {
        V
    }
    const fn default_u16<const V: u16>() -> u16 {
        V
    }

    //makes alloc in runtime =(
    fn default_str<const V: &'static str>() -> String {
        String::from(V)
    }

    #[derive(serde::Deserialize,Debug)]
    pub struct Conf {
        db: Db,
        mode: Mode,
        server: Server,
        log: Log,
        background: Background,
    }

    #[derive(serde::Deserialize,Debug)]
    pub struct Mode {
        #[serde(default = "default_bool::<false>")]
        debug: bool
    }

    #[derive(serde::Deserialize,Debug)]
    pub struct Server {
        #[serde(default = "default_str::<\"http://127.0.0.1\">")]
        external_url: String,
        #[serde(default = "default_u16::<8081>")]
        http_port: u16,
        #[serde(default = "default_u16::<8082>")]
        grpc_port: u16,
        #[serde(default = "default_u16::<10025>")]
        healthz_port: u16,
        #[serde(default = "default_u16::<9199>")]
        metrics_port: u16
    }

    #[derive(serde::Deserialize,Debug)]
    pub struct Db {
        mysql: Mysql
    }

    #[derive(serde::Deserialize,Debug)]
    pub struct Mysql {
        #[serde(default = "default_str::<\"http://127.0.0.1\">")]
        host: String,
        #[serde(default = "default_u16::<3306>")]
        port: u16,
        #[serde(default = "default_str::<\"http://127.0.0.1\">")]
        dating: String,
        #[serde(default = "default_str::<\"http://127.0.0.1\">")]
        user: String,
        #[serde(default = "default_str::<\"http://127.0.0.1\">")]
        pass: String,
        connections: Connections
    }

    #[derive(serde::Deserialize,Debug)]
    pub struct Connections {
        #[serde(default = "default_usize::<30>")]
        max_idle: usize,
        #[serde(default = "default_usize::<30>")]
        max_open: usize,
    }

    #[derive(serde::Deserialize,Debug)]
    pub struct Log {
        app: App
    }

    #[derive(serde::Deserialize,Debug)]
    pub struct App {
        #[serde(default = "default_str::<\"info\">")]
        level: String
    }

    #[derive(serde::Deserialize,Debug)]
    pub struct Background {
        watchdog: Watchdog
    }

    #[derive(serde::Deserialize,Debug)]
    pub struct Watchdog {
        #[serde(default = "default_str::<\"5s\">")]
        period: String,
        #[serde(default = "default_usize::<10>")]
        limit: usize,
        #[serde(default = "default_str::<\"4s\">")]
        lock_timeout: String,
    }
}

fn main() {
    let args = Opts::parse();

    let builder = Config::builder()
        .add_source(File::new(args.conf.as_deref().unwrap_or("config.toml"), FileFormat::Toml))
        .add_source(Environment::with_convert_case(config::Case::UpperSnake).prefix("CONF_"));

    let conf: conf::Conf = builder.build().unwrap().try_deserialize().unwrap();

    println!("Hello! the config is: {:?}", &conf);
}
