// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../src/arch/x86_64/acpi/tables/mcfg_types.rs"]
pub mod mcfg_types;

#[path = "../../../../../../../../../src/arch/x86_64/acpi/tables/sdt/mod.rs"]
pub mod sdt;

pub use mcfg_types::*;
pub use sdt::*;
