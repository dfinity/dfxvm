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
    if !plan.dfx_on_path.is_empty() {
        println!();
        println!("The following binaries were found on your PATH and will be deleted:");
        for binary in &plan.dfx_on_path {
            println!("   {}", binary.display());
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
    let delete_dfx_on_path = if options.delete_dfx_on_path {
        "yes"
    } else {
        "no"
    };
    let modify_path = if options.modify_path { "yes" } else { "no" };

    println!("Current installation options:");
    println!();
    println!("            dfx version: {}", style(dfx_version).bold());
    if !plan.dfx_on_path.is_empty() {
        println!(
            "     delete dfx on PATH: {}",
            style(delete_dfx_on_path).bold()
        );
    }
    println!("   modify PATH variable: {}", style(modify_path).bold());
    println!();
}

pub fn need_to_delete_old_dfx(plan: &Plan) {
    println!();
    println!("The following binaries could not be deleted:");
    for p in &plan.dfx_on_path {
        if p.exists() {
            println!("   {}", p.display());
        }
    }
    println!();
    println!(
        "You can either delete these files manually, or I can call {} for you,",
        style("sudo rm").bold()
    );
    println!("which will likely prompt you for your password.");
    println!();
}

pub fn success(plan: &Plan) {
    println!();
    println!("{}", style("dfxvm is installed now.").bold());
    println!();
    describe_legacy_binary_situation(plan);
    if plan.options.modify_path {
        post_install_msg_unix_modify_path(plan);
    } else {
        post_install_msg_unix_no_modify_path(plan);
    }
}

fn describe_legacy_binary_situation(plan: &Plan) {
    let remaining = plan
        .dfx_on_path
        .iter()
        .filter(|p| p.exists())
        .collect::<Vec<_>>();
    if !remaining.is_empty() {
        if remaining.len() == 1 {
            println!("The following dfx binary is still on your path:");
        } else {
            println!("The following dfx binaries are still on your path:");
        };
        for p in &remaining {
            println!("   {}", p.display());
        }
        if remaining.len() == 1 {
            println!("If you don't delete it, it may be called instead of");
        } else {
            println!("If you don't delete them, they may be called instead of");
        }
        println!("the dfx binary installed by dfxvm.");
        println!();
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
