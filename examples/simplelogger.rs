pub struct SimpleLogger;
impl ::log::Log for SimpleLogger {
    fn enabled(&self, _: &::log::LogMetadata) -> bool { true     }
    fn log(&self, record: &::log::LogRecord) { println!("{}", record.args());  }
}
