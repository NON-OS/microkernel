// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/arch/x86_64/uefi/constants/mod.rs"]
pub mod constants;

#[path = "../../../../../../../../src/arch/x86_64/uefi/types/mod.rs"]
pub mod types;

pub mod variable;

pub use constants::status;
pub use types::{Guid, ResetType, VariableAttributes};
pub use variable::{UefiVariable};
