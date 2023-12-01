use crate::dfxvm_init::plan::{DfxVersion, Plan};
use crate::error::dfxvm_init::InteractError;
use crate::log::log_error;
use semver::Version;

pub fn customize(mut plan: Plan) -> Result<Plan, InteractError> {
    println!("I'm going to ask you the value of each of these installation options.");
    println!("You may simply press the Enter key to leave unchanged.");
    println!();

    let dfx_version = select_dfx_version(&plan.dfx_version)?;
    plan = plan.with_dfx_version(dfx_version);
    println!();

    Ok(plan)
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
