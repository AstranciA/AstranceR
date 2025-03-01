use bitflags::bitflags;
use core::fmt::{self, Write};

use super::write;

struct Stdout;

const STDIN: usize = 0;
const STDOUT: usize = 1;

bitflags! {
    pub struct LogLevel: u8 {
        const TRACE = 0b00001;
        const DEBUG = 0b00010;
        const INFO  = 0b00100;
        const WARN  = 0b01000;
        const ERROR = 0b10000;
        const ALL   = Self::TRACE.bits() | Self::DEBUG.bits() | Self::INFO.bits() |
                     Self::WARN.bits() | Self::ERROR.bits();
    }
}

pub static LOG_LEVEL: LogLevel = LogLevel::from_bits_retain(LogLevel::ALL.bits());

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! log_macro {
    ($level:ident, $color:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        if $crate::console::LOG_LEVEL.contains($crate::console::LogLevel::$level) {
            $crate::console::print(format_args!(
                concat!("\x1b[", $color, "m[", stringify!($level), "] ", $fmt, "\x1b[0m\n"),
                $( $($arg)+ )?
            ));
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => (log_macro!(TRACE, "35", $($arg)*));
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => (log_macro!(DEBUG, "32", $($arg)*));
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (log_macro!(INFO, "34", $($arg)*));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (log_macro!(WARN, "33", $($arg)*));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (log_macro!(ERROR, "31", $($arg)*));
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let mut s = String::new();
        core::fmt::write(&mut s, format_args!($($arg)*)).unwrap();
        s
    }};
}
