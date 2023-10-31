use console::style;
use std::fmt;
use std::io::Write;

macro_rules! err {
    ($($arg:tt)*) => ( $crate::log::err_fmt(format_args!($($arg)*)) )
}

pub fn err_fmt(args: fmt::Arguments<'_>) {
    let mut stderr = std::io::stderr();
    let _ = write!(stderr, "{} ", style("error:").red().bold());
    let _ = stderr.write_fmt(args);
    let _ = writeln!(stderr);
}
