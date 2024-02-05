use crate::dfxvm_init::plan::{DfxVersion, Plan};
use crate::error::dfxvm_init::InteractError;
use crate::log::log_error;
use dialoguer::Confirm;
use semver::Version;

pub fn customize(plan: Plan) -> Result<Plan, InteractError> {
    println!("I'm going to ask you the value of each of these installation options.");
    println!("You may simply press the Enter key to leave unchanged.");
    println!();

    let mut options = plan.options.clone();

    let dfx_version = select_dfx_version(&options.dfx_version)?;
    options = options.with_dfx_version(dfx_version);

    if !plan.dfx_on_path.is_empty() {
        let delete_dfx_on_path = delete_dfx_on_path(options.delete_dfx_on_path)?;
        options = options.delete_dfx_on_path(delete_dfx_on_path);
    }

    let modify_path = select_modify_path(options.modify_path)?;
    options = options.with_modify_path(modify_path);

    println!();

    Ok(plan.with_options(options))
}

fn select_dfx_version(install_dfx: &DfxVersion) -> Result<DfxVersion, InteractError> {
    let default = match install_dfx {
        DfxVersion::Latest => "latest".to_string(),
        DfxVersion::Specific(version) => version.to_string(),
    };

    let dfx_version = loop {
        let s = dialoguer::Input::new()
            .with_prompt("dfx version?")
            .default(default.clone())
            .interact()?;

        if s == "latest" {
            break DfxVersion::Latest;
        } else {
            match Version::parse(&s) {
                Ok(version) => break DfxVersion::Specific(version),
                Err(e) => {
                    log_error(&e);
                    err!(r#"Please specify either a valid semver or "latest"."#);
                }
            }
        }
    };
    Ok(dfx_version)
}

fn delete_dfx_on_path(current: bool) -> Result<bool, InteractError> {
    let modify = Confirm::new()
        .with_prompt("Delete dfx binaries found on PATH?")
        .default(current)
        .interact()?;
    Ok(modify)
}

fn select_modify_path(current: bool) -> Result<bool, InteractError> {
    let modify = Confirm::new()
        .with_prompt("Modify PATH variable?")
        .default(current)
        .interact()?;
    Ok(modify)
}
