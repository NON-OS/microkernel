// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/arch/x86_64/uefi/constants/mod.rs"]
pub mod constants;

pub mod error;

pub mod types;

pub mod variable;

pub use constants::status;
pub use error::{UefiError, UefiResult};
pub use types::{Guid};
