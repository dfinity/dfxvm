use crate::dfxvm;
use crate::dfxvm_init::{
    plan::{DfxVersion, Plan},
    ui,
    ui::Confirmation,
};
use crate::error::{dfxvm_init, dfxvm_init::ExecutePlanError, fs::WriteFileError};
use crate::fs::create_dir_all;
use crate::installation::{env_file_contents, install_binaries};
use crate::locations::Locations;
use semver::Version;
use std::path::Path;

pub async fn initialize(
    dfx_version: Option<Version>,
    confirmation: Option<Confirmation>,
) -> Result<(), dfxvm_init::Error> {
    let locations = Locations::new()?;
    let mut plan = Plan::new(&locations);
    if let Some(version) = dfx_version {
        plan = plan.with_dfx_version(DfxVersion::Specific(version));
    }

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

    match &plan.dfx_version {
        DfxVersion::Latest => dfxvm::update().await?,
        DfxVersion::Specific(version) => dfxvm::set_default(version, locations).await?,
    }

    Ok(())
}

fn create_env_file(path: &Path) -> Result<(), WriteFileError> {
    info!("creating {}", path.display());
    crate::fs::write(path, env_file_contents())
}
