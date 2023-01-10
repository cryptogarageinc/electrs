use chrono::{SecondsFormat, Utc};
use log::{Level, Log, Metadata, Record, SetLoggerError};
use std::default::Default;
use std::fmt;
use std::io::Write;
use std::io::{self, Stdout};

#[derive(Serialize, Deserialize)]
struct JsonLogObj {
    timestamp: String,
    status: String,
    msg: String,
    caller: String,
}

#[derive(Debug, Clone)]
pub struct JsonLogger {
    level: Level,
}

impl JsonLogger {
    pub fn new() -> JsonLogger {
        JsonLogger::default()
    }

    pub fn verbosity(&mut self, verbosity: usize) -> &mut JsonLogger {
        self.level = match verbosity {
            0 => Level::Error,
            1 => Level::Warn,
            2 => Level::Info,
            3 => Level::Debug,
            _ => Level::Trace,
        };
        self
    }

    pub fn init(&mut self) -> Result<(), SetLoggerError> {
        let logger = StdOutJsonLogger {
            out: io::stdout(),
            level: self.level,
        };
        log::set_max_level(logger.level.to_level_filter());
        log::set_boxed_logger(Box::new(logger))
    }
}

impl fmt::Display for JsonLogger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JsonLogger{{\"level\":\"{}\"}}", &self.level)
    }
}

impl Default for JsonLogger {
    fn default() -> JsonLogger {
        JsonLogger {
            level: Level::Error,
        }
    }
}

struct StdOutJsonLogger {
    out: Stdout,
    level: Level,
}

impl Log for StdOutJsonLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(&record.metadata()) {
            return;
        }
        let line: u32 = match record.line() {
            Some(line) => line,
            _ => 0,
        };
        let caller = match record.file() {
            Some(file) => format!("{}:{}", file, line),
            _ => String::default(),
        };
        let obj = JsonLogObj {
            timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            status: std::format!("{}", record.level()),
            msg: record.args().to_string(),
            caller: caller,
        };
        if let Ok(s) = serde_json::to_string(&obj) {
            let _ = writeln!(self.out.lock(), "{}", s);
        }
    }

    fn flush(&self) {}
}
