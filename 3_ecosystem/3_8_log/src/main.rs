use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    sync::Mutex,
};

use once_cell::sync::OnceCell;

pub struct AppLog;

impl AppLog {
    pub fn init() {
        static APP_LOG: AppLog = AppLog;

        log::set_logger(&APP_LOG)
            .map(|()| log::set_max_level(log::LevelFilter::Info))
            .expect("cannot set logger");
    }
}

fn make_json_message<M: AsRef<str>>(
    lvl: log::Level,
    rec: &log::Record,
    msg: M,
) -> serde_json::Value {
    let value = serde_json::json!({
        "lvl": lvl.as_str(),
        "time": format!("{:?}",std::time::Instant::now()),
        "file": rec.file(),
        "msg": msg.as_ref()
    });
    value
}

impl log::Log for AppLog {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        // panic!();
        if record.level() > log::Level::Warn {
            let msg =
                make_json_message(record.level(), record, record.args().as_str().unwrap_or(""));
            let _ = writeln!(std::io::stderr(), "{}", msg);
        } else {
            let msg =
                make_json_message(record.level(), record, record.args().as_str().unwrap_or(""));
            let _ = writeln!(std::io::stdout(), "{}", msg);
        }
    }

    fn flush(&self) {
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
    }
}

pub struct AccessLog(OnceCell<(Mutex<BufWriter<File>>, String)>);

impl AccessLog {
    pub fn init<P: AsRef<Path>>(to: P) {
        let file = File::create(to.as_ref()).unwrap();
        let fname = to.as_ref().file_name().unwrap();

        static INSTANCE: AccessLog = AccessLog(OnceCell::new());

        INSTANCE.0.get_or_init(|| {
            (
                Mutex::new(BufWriter::new(file)),
                fname.to_str().expect("cannot name a file").to_owned(),
            )
        });

        log::set_logger(&INSTANCE)
            .map(|()| log::set_max_level(log::LevelFilter::Info))
            .expect("cannot set logger");
    }

    fn write_message(&self, val: serde_json::Value) {
        self.0
            .get()
            .expect("cannot get")
            .0
            .lock()
            .map(|mut g| {
                let _ = writeln!(&mut g, "{}", val);
                let _ = g.flush();
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
        let msg = make_json_message(record.level(), record, s);
        self.write_message(msg)
    }

    fn flush(&self) {
        self.0
            .get()
            .expect("cannot get")
            .0
            .lock()
            .map(|mut f| f.flush().expect("cannot flush"))
            .expect("cannot lock");
    }
}

const PATH: &str = "./access.log";

fn main() {}

#[cfg(test)]
mod tests {

    use crate::{AccessLog, AppLog, PATH};

    #[test]
    fn test_first_logger() {
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
