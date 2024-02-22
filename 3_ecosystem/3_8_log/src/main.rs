use std::{
    fmt::format, fs::File, io::{BufWriter, Write}, iter::Once, mem::zeroed, path::{Path, PathBuf}, sync::Mutex
};

use once_cell::sync::OnceCell;

struct AppLog;

impl AppLog {
    fn init() {
        log::set_logger(&AppLog).expect("cannot set logger");
    }
}

 fn make_json_message<M: AsRef<str>>(lvl: log::Level,rec: &log::Record, msg: M) -> serde_json::Value {
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
        if record.level() > log::Level::Warn {
            let msg = make_json_message(record.level(), record, record.args().as_str().unwrap_or(""));
            write!(std::io::stderr(),"{}",msg.to_string()).unwrap();
        } else {
            let msg = make_json_message(record.level(), record, record.args().as_str().unwrap_or(""));
            write!(std::io::stdout(),"{}",msg.to_string()).unwrap();
        }
    }

    fn flush(&self) {
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
    }
}

struct AccessLog(OnceCell<(Mutex<BufWriter<File>>, String)>);

impl AccessLog {
    fn init<P: AsRef<Path>>(to: P) {
        let file = File::create(to.as_ref()).unwrap();
        let fname = to.as_ref().file_name().unwrap();
        // now mutex holds absolutely invalid instance which we cannot create
        static INSTANCE: AccessLog = AccessLog(OnceCell::new());

        INSTANCE.0.get_or_init(|| (
            Mutex::new(BufWriter::new(file)),
            fname.to_str().expect("cannot name a file").to_owned()
        ));


        log::set_logger(&INSTANCE).expect("cannot set logger");
    }

    fn write_message<M: AsRef<str>>(&self,val: serde_json::Value) {
        self
            .0
            .get()
            .expect("cannot get")
            .0
            .lock()
            .map(|mut g| {
                writeln!(&mut g,"{}", val.to_string()).expect("cannot write the message");
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
        self.write_message::<&str>(make_json_message(record.level(), record, s))
    }

    fn flush(&self) {
        self
            .0
            .get()
            .expect("cannot get")
            .0
            .lock()
            .map(|mut f| f.flush().expect("cannot flush"))
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
