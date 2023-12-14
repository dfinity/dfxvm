use crate::dfxvm;
use crate::dfxvm_init::{
    plan::{DfxVersion, Plan, PlanOptions},
    ui,
    ui::Confirmation,
};
use crate::error::{
    dfxvm_init,
    dfxvm_init::{ExecutePlanError, UpdateProfileScriptsError},
    fs::WriteFileError,
};
use crate::fs::{append_to_file, create_dir_all, read_to_string};
use crate::installation::{env_file_contents, install_binaries, ProfileScript};
use crate::locations::Locations;
use std::path::Path;

pub async fn initialize(
    options: PlanOptions,
    confirmation: Option<Confirmation>,
) -> Result<(), dfxvm_init::Error> {
    let locations = Locations::new()?;
    let mut plan = Plan::new(options, &locations);

    ui::display::introduction(&plan);

    let plan = loop {
        ui::display::options(&plan);
        let confirmation = confirmation.map_or_else(ui::confirm, Ok)?;
        match confirmation {
            Confirmation::Proceed => break Some(plan),
            Confirmation::Customize => plan = ui::customize(plan)?,
            Confirmation::Cancel => break None,
        }
    };

    let Some(plan) = plan else {
        info!("aborting installation");
        return Ok(());
    };

    execute(&plan, &locations).await?;

    ui::display::success(&plan);

    Ok(())
}

pub async fn execute(plan: &Plan, locations: &Locations) -> Result<(), ExecutePlanError> {
    create_dir_all(&plan.bin_dir)?;

    create_env_file(&plan.env_path)?;

    install_binaries(&plan.bin_dir)?;

    match &plan.options.dfx_version {
        DfxVersion::Latest => dfxvm::update().await?,
        DfxVersion::Specific(version) => dfxvm::set_default(version, locations).await?,
    }

    if plan.options.modify_path {
        update_profile_scripts(&plan.profile_scripts)?;
    }

    Ok(())
}

fn create_env_file(path: &Path) -> Result<(), WriteFileError> {
    info!("creating {}", path.display());
    crate::fs::write(path, env_file_contents())
}

fn update_profile_scripts(
    profile_scripts: &Vec<ProfileScript>,
) -> Result<(), UpdateProfileScriptsError> {
    for profile_script in profile_scripts {
        let path = &profile_script.path;
        let rc = if path.exists() {
            read_to_string(&profile_script.path)?
        } else {
            "".to_string()
        };

        let source_command = profile_script.source_string();
        if rc.contains(&source_command) {
            info!("already updates path: {}", path.display());
            continue;
        }

        info!("updating {}", path.display());

        let source_to_append = if rc.ends_with('\n') || rc.is_empty() {
            source_command
        } else {
            format!("\n{}", source_command)
        };

        append_to_file(&profile_script.path, &source_to_append)?;
    }
    Ok(())
}
