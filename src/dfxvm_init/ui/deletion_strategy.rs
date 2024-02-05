use crate::error::dfxvm_init::InteractError;
use dialoguer::Select;

pub enum DeletionStrategy {
    Manual,
    CallSudo,
    DontDelete,
}

pub fn select_deletion_strategy() -> Result<DeletionStrategy, InteractError> {
    let items = vec![
        "I've deleted them manually (default)",
        "Call sudo rm for me (I'll enter my password)",
        "Don't delete anything. I'll do it later",
    ];

    let index = Select::new()
        .with_prompt("How would you like to proceed?")
        .default(0)
        .items(&items)
        .interact()?;

    let deletion_strategy = match index {
        0 => DeletionStrategy::Manual,
        1 => DeletionStrategy::CallSudo,
        2 => DeletionStrategy::DontDelete,
        _ => unreachable!(),
    };

    Ok(deletion_strategy)
}
