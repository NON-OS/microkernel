// NONOS Operating System (AGPL-3.0-or-later)

pub mod fadt;

#[path = "../../../../../../../../../src/arch/x86_64/acpi/tables/sdt/mod.rs"]
pub mod sdt;

pub use fadt::*;
pub use sdt::*;
