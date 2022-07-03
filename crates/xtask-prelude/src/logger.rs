use std::io::Write;

use log::{Level, LevelFilter, Log};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

static LOGGER: Logger = Logger;
pub struct Logger;

impl Logger {
    pub fn init() {
        if let Err(err) = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info)) {
            println!("{err:?}");
            std::process::exit(1);
        }
    }
}

impl Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let writer = BufferWriter::stdout(ColorChoice::Auto);
            let mut buffer = writer.buffer();
            let no_color = ColorSpec::new();

            // level
            let level = record.level();
            let level_color = match level {
                Level::Trace => None,
                Level::Debug => Some(Color::Cyan),
                Level::Info => Some(Color::Green),
                Level::Warn => Some(Color::Yellow),
                Level::Error => Some(Color::Red),
            };
            let mut color_spec = ColorSpec::new();
            color_spec.set_fg(level_color);
            buffer.set_color(&color_spec).expect("Fail to set_color");
            write!(&mut buffer, "{:>5}", level.as_str()).expect("Fail to write");

            buffer.set_color(&no_color).expect("Fail to set_color");
            write!(&mut buffer, " ").expect("Fail to write");

            // target
            let mut target_color = ColorSpec::new();
            target_color.set_dimmed(true);
            buffer.set_color(&target_color).expect("Fail to set_color");
            write!(&mut buffer, "{}:", record.target()).expect("Fail to write");

            buffer.set_color(&no_color).expect("Fail to set_color");
            write!(&mut buffer, " ").expect("Fail to write");

            // args
            writeln!(&mut buffer, "{}", record.args()).expect("Fail to write");

            writer.print(&buffer).expect("Fail to print");
        }
    }

    fn flush(&self) {}
}
