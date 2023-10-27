use console::style;
use std::error::Error;
use std::fmt;
use std::io::Write;

macro_rules! err {
    ($($arg:tt)*) => ( $crate::log::err_fmt(format_args!($($arg)*)) )
}

macro_rules! info {
    ($($arg:tt)*) => ( $crate::log::info_fmt(format_args!($($arg)*)) )
}

pub fn err_fmt(args: fmt::Arguments<'_>) {
    let mut stderr = std::io::stderr();
    let _ = write!(stderr, "{} ", style("error:").red().bold());
    let _ = stderr.write_fmt(args);
    let _ = writeln!(stderr);
}

pub fn info_fmt(args: fmt::Arguments<'_>) {
    let mut stderr = std::io::stderr();
    let _ = write!(stderr, "{} ", style("info:").bold());
    let _ = stderr.write_fmt(args);
    let _ = writeln!(stderr);
}

pub fn log_error(e: &dyn Error) {
    err!("{:#}", e);
    let mut source = e.source();
    while let Some(cur) = source {
        err!("    caused by: {:#}", cur);
        source = cur.source();
    }
}
