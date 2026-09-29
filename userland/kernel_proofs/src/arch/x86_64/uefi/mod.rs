// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../src/arch/x86_64/uefi/constants/mod.rs"]
pub mod constants;

#[path = "../../../../../../src/arch/x86_64/uefi/crc.rs"]
pub mod crc;

pub mod error;

pub mod manager;

#[path = "../../../../../../src/arch/x86_64/uefi/stats.rs"]
pub mod stats;

pub mod tables;

#[path = "../../../../../../src/arch/x86_64/uefi/types/mod.rs"]
pub mod types;

#[path = "../../../../../../src/arch/x86_64/uefi/variable/mod.rs"]
pub mod variable;

pub use constants::status;
pub use crc::{compute as crc32_compute, Crc32};
pub use error::{UefiError, UefiResult};
pub use manager::{is_initialized, UefiManager, UEFI_MANAGER};
pub use stats::UefiStats;
pub use tables::{EfiTime, EfiTimeCapabilities, RuntimeServices, TableHeader};
pub use types::{Guid, ResetType, VariableAttributes};
pub use variable::{FirmwareInfo, UefiVariable};
