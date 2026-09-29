// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../src/arch/x86_64/uefi/manager/core.rs"]
pub mod core;

#[path = "../../../../../../../src/arch/x86_64/uefi/manager/init.rs"]
pub mod init;

#[path = "../../../../../../../src/arch/x86_64/uefi/manager/state.rs"]
pub mod state;

#[path = "../../../../../../../src/arch/x86_64/uefi/manager/variable.rs"]
pub mod variable;

#[path = "../../../../../../../src/arch/x86_64/uefi/manager/variable_raw.rs"]
pub mod variable_raw;

pub use core::UefiManager;
pub use state::{is_initialized, UEFI_MANAGER};
