use crate::err;
use crate::errors::cli::DetermineModeError::{NoExeName, UnrecognizedExeName};
use crate::errors::cli::{DetermineModeError, DispatchError};
use crate::{dfx, dfxvm, dfxvm_init};
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;
use Mode::{Init, Manage, Proxy};

pub fn main(args: &[OsString]) -> ExitCode {
    dispatch(args).unwrap_or_else(|err| {
        report_error(err);
        ExitCode::FAILURE
    })
}

fn dispatch(args: &[OsString]) -> Result<ExitCode, DispatchError> {
    let exit_code = match determine_mode(args)? {
        Manage => dfxvm::main(args)?,
        Init => dfxvm_init::main(args)?,
        Proxy => dfx::main(args)?,
    };
    Ok(exit_code)
}

enum Mode {
    Init,
    Manage,
    Proxy,
}

fn determine_mode(args: &[OsString]) -> Result<Mode, DetermineModeError> {
    let process_name = get_program_name(args);

    match process_name.as_deref() {
        Some("dfx") => Ok(Proxy),
        Some("dfxvm") => Ok(Manage),
        Some(n) if n.starts_with("dfxvm-init") => {
            // NB: The above check is only for the prefix of the file
            // name. Browsers rename duplicates to
            // e.g. dfxvm-init(2), and this allows all variations
            // to work.
            Ok(Init)
        }
        Some(other) => Err(UnrecognizedExeName(other.to_string())),
        None => Err(NoExeName),
    }
}

fn get_program_name(args: &[OsString]) -> Option<String> {
    args.first()
        .map(PathBuf::from)
        .as_ref()
        .and_then(|p| p.file_stem())
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
}

fn report_error(e: DispatchError) {
    err!("{:#}", e);
}
