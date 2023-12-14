use crate::dfxvm_init::plan::{DfxVersion, Plan};
use console::style;

// Most of the text in this file is derived/copied/modified from:
// https://github.com/rust-lang/rustup/blob/master/src/cli/self_update.rs

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
    if !plan.profile_scripts.is_empty() {
        println!();
        println!("This path will then be added to your PATH environment variable by");
        println!("modifying the profile files located at:");
        println!();

        for script in &plan.profile_scripts {
            println!("   {}", script.path.display());
        }
    }
    println!();
}

pub fn options(plan: &Plan) {
    let options = &plan.options;
    let dfx_version = match &options.dfx_version {
        DfxVersion::Latest => "latest".to_string(),
        DfxVersion::Specific(version) => version.to_string(),
    };
    let modify_path = if options.modify_path { "yes" } else { "no" };

    println!("Current installation options:");
    println!();
    println!("            dfx version: {}", style(dfx_version).bold());
    println!("   modify PATH variable: {}", style(modify_path).bold());
    println!();
}

pub fn success(plan: &Plan) {
    println!();
    println!("{}", style("dfxvm is installed now.").bold());
    println!();
    if plan.options.modify_path {
        post_install_msg_unix_modify_path(plan);
    } else {
        post_install_msg_unix_no_modify_path(plan);
    }
}

pub fn post_install_msg_unix_modify_path(plan: &Plan) {
    println!("To get started you may need to restart your current shell.");
    println!("This would reload your PATH environment variable to include");
    println!("the dfxvm bin directory.");
    println!();
    post_install_msg_unix_configure_shell(plan);
}

pub fn post_install_msg_unix_no_modify_path(plan: &Plan) {
    println!("To get started you need the dfxvm bin directory in your PATH:");
    println!("  {}", style(&plan.bin_dir.display()).bold());
    println!("This has not been done automatically.");
    println!();
    post_install_msg_unix_configure_shell(plan);
}

pub fn post_install_msg_unix_configure_shell(plan: &Plan) {
    println!("To configure your shell, run:");
    println!(r#"  source "{}""#, plan.env_path_user_facing);
    println!();
}
