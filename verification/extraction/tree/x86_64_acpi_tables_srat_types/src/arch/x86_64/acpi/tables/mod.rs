// NONOS Operating System (AGPL-3.0-or-later)

pub mod sdt;

#[path = "../../../../../../../../../src/arch/x86_64/acpi/tables/srat_types.rs"]
pub mod srat_types;

pub use sdt::*;
pub use srat_types::*;
