use crate::dfxvm_init::plan::{DfxVersion, Plan};
use console::style;

pub fn introduction(plan: &Plan) {
    println!();
    println!("{}", style("Welcome to dfxvm!").bold());
    println!();
    println!("This will install dfxvm, and download and install dfx.");
    println!();
    println!(
        "The {} and {} commands will be added to the following directory:",
        style("dfxvm").bold(),
        style("dfx").bold()
    );
    println!();
    println!("   {}", plan.bin_dir.display());
    println!();
}

pub fn options(plan: &Plan) {
    let options = &plan.options;
    let dfx_version = match &options.dfx_version {
        DfxVersion::Latest => "latest".to_string(),
        DfxVersion::Specific(version) => version.to_string(),
    };

    println!("Current installation options:");
    println!();
    println!("   dfx version: {}", style(dfx_version).bold());
    println!();
}

pub fn success(plan: &Plan) {
    println!();
    println!("{}", style("dfxvm is installed now.").bold());
    println!();
    println!("The installation process doesn't yet update profile scripts");
    println!("to add the dfxvm bin directory to your $PATH.");
    println!();
    println!("To configure your shell, run:");
    println!(r#"  source "{}""#, plan.env_path_user_facing);
    println!();
}
