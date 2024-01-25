use crate::env::home_dir;
use crate::installation::profile::ProfileScript;
use std::path::PathBuf;
use std::process::Command;

// Most of this is copied/derived/modified from
// https://github.com/rust-lang/rustup/blob/master/src/cli/self_update/shell.rs

pub(crate) type Shell = Box<dyn UnixShell>;

fn enumerate_shells() -> Vec<Shell> {
    vec![Box::new(Posix), Box::new(Bash), Box::new(Zsh)]
}

fn get_available_shells() -> impl Iterator<Item = Shell> {
    enumerate_shells().into_iter().filter(|sh| sh.does_exist())
}

pub fn get_detected_profile_scripts() -> Vec<ProfileScript> {
    get_available_shells()
        .flat_map(|sh| sh.update_rcs())
        .collect()
}

pub fn get_all_profile_scripts() -> Vec<ProfileScript> {
    enumerate_shells()
        .iter()
        .flat_map(|sh| sh.rcfiles())
        .collect()
}

pub(crate) trait UnixShell {
    // Detects if a shell "exists". Users have multiple shells, so an "eager"
    // heuristic should be used, assuming shells exist if any traces do.
    fn does_exist(&self) -> bool;

    // Gives all rcfiles of a given shell that dfxvm is concerned with.
    // Used primarily in checking rcfiles for cleanup.
    fn rcfiles(&self) -> Vec<ProfileScript>;

    // Gives rcs that should be written to.
    fn update_rcs(&self) -> Vec<ProfileScript>;
}

struct Posix;
impl UnixShell for Posix {
    fn does_exist(&self) -> bool {
        true
    }

    fn rcfiles(&self) -> Vec<ProfileScript> {
        match home_dir() {
            Ok(dir) => vec![ProfileScript::posix(dir.join(".profile"))],
            _ => vec![],
        }
    }

    fn update_rcs(&self) -> Vec<ProfileScript> {
        // Write to .profile even if it doesn't exist. It's the only rc in the
        // POSIX spec so it should always be set up.
        self.rcfiles()
    }
}

struct Bash;

impl UnixShell for Bash {
    fn does_exist(&self) -> bool {
        !self.update_rcs().is_empty()
    }

    fn rcfiles(&self) -> Vec<ProfileScript> {
        let home = home_dir().ok();
        // Bash also may read .profile, however UnixShell for Posix already handles this
        [".bash_profile", ".bash_login", ".bashrc"]
            .iter()
            .filter_map(|rc| home.clone().map(|dir| dir.join(rc)))
            .map(ProfileScript::posix)
            .collect()
    }

    fn update_rcs(&self) -> Vec<ProfileScript> {
        self.rcfiles()
            .into_iter()
            .filter(|rc| rc.is_file())
            .collect()
    }
}

struct Zsh;

impl Zsh {
    fn zdotdir() -> Option<PathBuf> {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        if matches!(std::env::var("SHELL"), Ok(sh) if sh.contains("zsh")) {
            match std::env::var_os("ZDOTDIR") {
                Some(dir) if !dir.is_empty() => Some(PathBuf::from(dir)),
                _ => None,
            }
        } else {
            match Command::new("zsh")
                .args(["-c", "echo -n $ZDOTDIR"])
                .output()
            {
                Ok(io) if !io.stdout.is_empty() => {
                    Some(PathBuf::from(OsStr::from_bytes(&io.stdout)))
                }
                _ => None,
            }
        }
    }
}

impl UnixShell for Zsh {
    fn does_exist(&self) -> bool {
        // zsh has to either be the shell or be callable for zsh setup.
        matches!(std::env::var("SHELL"), Ok(sh) if sh.contains("zsh")) || find_cmd("zsh")
    }

    fn rcfiles(&self) -> Vec<ProfileScript> {
        [Zsh::zdotdir(), home_dir().ok()]
            .iter()
            .filter_map(|dir| dir.as_ref().map(|p| p.join(".zshenv")))
            .map(ProfileScript::posix)
            .collect()
    }

    fn update_rcs(&self) -> Vec<ProfileScript> {
        // zsh can change $ZDOTDIR both _before_ AND _during_ reading .zshenv,
        // so we: write to $ZDOTDIR/.zshenv if-exists ($ZDOTDIR changes before)
        // OR write to $HOME/.zshenv if it exists (change-during)
        // if neither exist, we create one of them ourselves:
        //    - $ZDOTDIR/.zshenv if $ZDOTDIR is set
        //    - $HOME/.zshenv otherwise
        self.rcfiles()
            .into_iter()
            .filter(|env| env.is_file())
            .chain(self.rcfiles())
            .take(1)
            .collect()
    }
}

fn find_cmd(cmd: &str) -> bool {
    let path = std::env::var_os("PATH").unwrap_or_default();
    std::env::split_paths(&path)
        .map(|dir| dir.join(cmd))
        .any(|p| p.exists())
}
