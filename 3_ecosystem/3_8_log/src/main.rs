use std::{
    fmt::format,
    fs::File,
    io::Write,
    mem::zeroed,
    path::{Path, PathBuf},
    sync::Mutex,
    io::BufWriter
};

struct AppLog;

impl AppLog {
    fn init() {
        log::set_logger(&AppLog).expect("cannot set logger");
    }
}

impl log::Log for AppLog {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if record.level() > log::Level::Warn {
            std::io::stderr()
                .write_fmt(*record.args())
                .expect("cannot write to stderr");
        } else {
            std::io::stdout()
                .write_fmt(*record.args())
                .expect("cannot write to stdout");
        }
    }

    fn flush(&self) {
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
    }
}

struct AccessLog(Mutex<(BufWriter<File>, String)>);

impl AccessLog {
    fn init<P: AsRef<Path>>(to: P) {
        let file = File::create(to.as_ref()).unwrap();
        let fname = to.as_ref().file_name().unwrap();
        // now mutex holds absolutely invalid instance
        static INSTANCE: AccessLog = Self(Mutex::new((unsafe { zeroed() }, String::new())));

        // we insert a valid instance there
        INSTANCE
            .0
            .lock()
            .map(|mut g| {
                g.0 = BufWriter::new(file);
                g.1 = fname.to_str().expect("cannot name a file").to_owned()
            })
            .expect("cannot lock empty mutex");

        log::set_logger(&INSTANCE).expect("cannot set logger");
    }

    fn write_message<M: AsRef<str>>(&self, lvl: log::Level, msg: M) {
        self.0
            .lock()
            .map(|mut g| {
                let value = serde_json::json!({
                    "lvl": lvl.as_str(),
                    "time": format!("{:?}",std::time::Instant::now()),
                    "file": g.1,
                    "msg": msg.as_ref()
                });
                writeln!(&mut g.0,"{}", value).expect("cannot write the message");
            })
            .expect("death");
    }
}

impl log::Log for AccessLog {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let Some(s) = record.args().as_str() else {
            panic!("cannot get the data")
        };
        self.write_message(record.level(), s)
    }

    fn flush(&self) {
        self.0
            .lock()
            .map(|mut f| f.0.flush().expect("cannot flush"))
            .expect("cannot lock");
    }
}

fn main() {}

#[cfg(test)]
mod tests {

    use crate::{AccessLog, AppLog};

    const PATH: &'static str = "./access.log";

    #[test]
    fn test_fisrt_logger() {
        AppLog::init();

        log::trace!("trace");

        log::debug!("debug");

        log::info!("info");

        log::warn!("warn");

        log::error!("err");
    }

    #[test]
    fn test_second_logger() {
        AccessLog::init(PATH);

        log::trace!("trace");

        log::debug!("debug");

        log::info!("info");

        log::warn!("warn");

        log::error!("err");
    }
}
