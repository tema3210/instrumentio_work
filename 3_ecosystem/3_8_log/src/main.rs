use std::{
    fs::File, io::{BufWriter, Write}, ops::Range, path::Path, sync::Mutex
};

use log::Level;

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

fn make_json_message(
    lvl: log::Level,
    msg: impl AsRef<str>,
    target: impl AsRef<str>
) -> serde_json::Value {
    let value = serde_json::json!({
        "lvl": lvl.as_str(),
        "time": format!("{:?}",std::time::Instant::now()),
        "file": target.as_ref(),
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
            let msg =
                make_json_message(record.level(), record.args().as_str().unwrap_or(""),"app.log");
            let _ = writeln!(std::io::stderr(), "{}", msg);
        } else {
            let msg =
                make_json_message(record.level(), record.args().as_str().unwrap_or(""),"app.log");
            let _ = writeln!(std::io::stdout(), "{}", msg);
        }
    }

    fn flush(&self) {
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
    }
}

type AccessLogInner = (Mutex<BufWriter<File>>, String);
pub struct AccessLog(OnceCell<AccessLogInner>);

impl AccessLog {

    pub fn make_inner<P: AsRef<Path>>(to: P) -> AccessLogInner {
        let file = File::create(to.as_ref()).unwrap();
        let fname = to.as_ref().file_name().unwrap();
        (
            Mutex::new(BufWriter::new(file)),
            fname.to_str().expect("cannot name a file").to_owned(),
        )
    }

    pub fn make_instance(inner: AccessLogInner) -> &'static Self {
        static INSTANCE: AccessLog = AccessLog(OnceCell::new());

        INSTANCE.0.get_or_init(|| inner);

        &INSTANCE
    }


    pub fn init<P: AsRef<Path>>(to: P) {
        log::set_logger(Self::make_instance(Self::make_inner(to)))
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
        let msg = make_json_message(record.level(), s, "access.log");
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


pub struct Combinator {
    prev: Option<Box<Self>>,
    current: &'static dyn log::Log,
    range: Range<Level>
}

impl Combinator {

    pub fn init(self, ll: log::LevelFilter) {
        let b = Box::new(self);
        log::set_boxed_logger(b)
        .map(|()| log::set_max_level(ll))
        .expect("cannot set logger");
    }

    pub fn wrap(c: &'static dyn log::Log, range: Range<Level>) -> Self {
        assert!(!range.is_empty());
        Self {
            prev: None,
            current: c,
            range
        }
    }

    ///there used to be also unchain method to pop the logger from this list, but log crate offers no support for swapping loggers, and in fact prevents us from doing so
    pub fn chain(self,next: &'static dyn log::Log, range: Range<Level>) -> Self {
        assert!(!range.is_empty());
        Self {
            prev: Some(Box::new(self)),
            current: next,
            range
        }
    }

}

impl log::Log for Combinator {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {

        let should_log = self.range.contains(&record.level());

        if should_log {
            //delegate to current in case we need to
            self.current.log(record)
        }

        // and propagate to previous
        if let Some(prev) = &self.prev {
            prev.log(record)
        }
    }

    fn flush(&self) {
        self.current.flush();

        // and propagate to previous
        if let Some(prev) = &self.prev {
            prev.flush()
        }
    }
}

pub const PATH: &str = "./access.log";

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{AccessLog, AppLog, Combinator, PATH};
    use log::Level;

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

    #[test]
    fn test_both_loggers() {
        Combinator::wrap(
            AccessLog::make_instance(AccessLog::make_inner(PATH)),
            Level::Error..Level::Warn
        ).chain(
            &AppLog,
            Level::Warn..Level::Trace
        )
        .init(log::LevelFilter::max());

        log::trace!("trace");

        log::debug!("debug");

        log::info!("info");

        log::warn!("warn");

        log::error!("err");
    }
}
