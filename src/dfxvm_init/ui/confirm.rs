use crate::error::dfxvm_init::InteractError;
use dialoguer::Select;

#[derive(Copy, Clone)]
pub enum Confirmation {
    Proceed,
    Customize,
    Cancel,
}

pub fn confirm() -> Result<Confirmation, InteractError> {
    let items = vec![
        "Proceed with installation (default)",
        "Customize installation",
        "Cancel installation",
    ];

    let index = Select::new()
        .with_prompt("Proceed with installation?")
        .default(0)
        .items(&items)
        .interact()?;

    println!();

    let confirmation = match index {
        0 => Confirmation::Proceed,
        1 => Confirmation::Customize,
        2 => Confirmation::Cancel,
        _ => unreachable!(),
    };

    Ok(confirmation)
}
