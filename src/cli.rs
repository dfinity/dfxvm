use crate::error::cli::DetermineModeError::{NoExeName, UnrecognizedExeName};
use crate::error::cli::{DetermineModeError, DispatchError};
use crate::locations::Locations;
use crate::log::log_error;
use crate::{dfx, dfxvm, dfxvm_init};
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;
use Mode::{Init, Manage, Proxy};

pub async fn main(args: &[OsString]) -> ExitCode {
    dispatch(args).await.unwrap_or_else(|err| {
        log_error(&err);
        ExitCode::FAILURE
    })
}

pub async fn dispatch(args: &[OsString]) -> Result<ExitCode, DispatchError> {
    let locations = Locations::new()?;
    let exit_code = match determine_mode(args)? {
        Init => dfxvm_init::main(args, &locations).await?,
        Manage => dfxvm::main(args, &locations).await?,
        Proxy => dfx::main(args, &locations)?,
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
