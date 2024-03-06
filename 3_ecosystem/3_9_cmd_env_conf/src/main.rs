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
    #[derive(serde::Deserialize,serde::Serialize,Debug,smart_default::SmartDefault)]
    pub struct Conf {
        db: Db,
        mode: Mode,
        server: Server,
        log: Log,
        background: Background,
    }

    #[derive(serde::Deserialize,serde::Serialize,Debug,smart_default::SmartDefault)]
    pub struct Mode {
        #[default = false]
        debug: bool
    }

    #[derive(serde::Deserialize,serde::Serialize,Debug,smart_default::SmartDefault)]
    pub struct Server {
        #[default = "http://127.0.0.1"]
        external_url: String,
        #[default = 8801]
        http_port: u16,
        #[default = 8802]
        grpc_port: u16,
        #[default = 10025]
        healthz_port: u16,
        #[default = 9199]
        metrics_port: u16
    }

    #[derive(serde::Deserialize,serde::Serialize,Debug,smart_default::SmartDefault)]
    pub struct Db {
        mysql: Mysql
    }

    #[derive(serde::Deserialize,serde::Serialize,Debug,smart_default::SmartDefault)]
    pub struct Mysql {
        #[default = "http://127.0.0.1"]
        host: String,
        #[default = 3306]
        port: u16,
        #[default = "default"]
        dating: String,
        #[default = "root"]
        user: String,
        #[default = ""]
        pass: String,
        connections: Connections
    }

    #[derive(serde::Deserialize,serde::Serialize,Debug,smart_default::SmartDefault)]
    pub struct Connections {
        #[default = 30]
        max_idle: usize,
        #[default = 30]
        max_open: usize,
    }

    #[derive(serde::Deserialize,serde::Serialize,Debug,smart_default::SmartDefault)]
    pub struct Log {
        app: App
    }

    #[derive(serde::Deserialize,serde::Serialize,Debug,smart_default::SmartDefault)]
    pub struct App {
        #[default = "info"]
        level: String
    }

    #[derive(serde::Deserialize,serde::Serialize,Debug,smart_default::SmartDefault)]
    pub struct Background {
        watchdog: Watchdog
    }

    #[derive(serde::Deserialize,serde::Serialize,Debug,smart_default::SmartDefault)]
    pub struct Watchdog {
        #[default = "5s"]
        period: String,
        #[default = 10]
        limit: usize,
        #[default = "4s"]
        lock_timeout: String,
    }
}

fn main() {
    let args = Opts::parse();

    let builder = Config::builder()
        .add_source(
            Config::try_from(&conf::Conf::default()).expect("cannot process default values")
        )
        .add_source(File::new(args.conf.as_deref().unwrap_or("config.toml"), FileFormat::Toml))
        .add_source(Environment::with_convert_case(config::Case::UpperSnake).prefix("CONF_"));

    let conf: conf::Conf = builder.build().unwrap().try_deserialize().unwrap();

    println!("Hello! the config is: {:#?}", &conf);
}
