use core::fmt;

use log::{self, Level, LevelFilter, Log, Metadata, Record};
use core::fmt::Write;

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Info,
    });
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            crate::sbi::console_putchar(c as usize);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::console::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Add escape sequence to print with color in Linux console
macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    }};
}

fn print_in_color(args: fmt::Arguments, color_code: u8) {
    Stdout.write_fmt(with_color!(args, color_code)).unwrap();
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        /*
        if let Some(tid) = processor().tid_option() {
            print_in_color(
                format_args!(
                    "[{:>5}][{},{}] {}\n",
                    record.level(),
                    crate::arch::cpu::id(),
                    tid,
                    record.args()
                ),
                level_to_color_code(record.level()),
            );
        } else {
            */
        print_in_color(
            format_args!(
                // "[{:>5}][{},-] {}\n",
                "[{:>5}] {}\n",
                record.level(),
                // crate::arch::cpu::id(),
                record.args()
            ),
            level_to_color_code(record.level()),
        );
        //}
    }
    fn flush(&self) {}
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}
