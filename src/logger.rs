use colored::Colorize;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let pretty_level = match record.level() {
                Level::Trace => "[.]".white(),
                Level::Debug => "[d]".white(),
                Level::Info => "[i]".bright_green(),
                Level::Warn => "[W]".yellow(),
                Level::Error => "[E]".red(),
            };
            println!("{} {}", pretty_level, record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
}
