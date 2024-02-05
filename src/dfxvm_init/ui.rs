mod confirm;
mod customize;
mod deletion_strategy;
pub mod display;

pub use confirm::confirm;
pub use confirm::Confirmation;
pub use customize::customize;
pub use deletion_strategy::{select_deletion_strategy, DeletionStrategy};
