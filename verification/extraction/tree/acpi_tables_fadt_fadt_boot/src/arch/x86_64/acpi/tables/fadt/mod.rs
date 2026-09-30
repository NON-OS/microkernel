// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/fadt/fadt_boot.rs"]
pub mod fadt_boot;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/fadt/fadt_struct.rs"]
pub mod fadt_struct;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/fadt/flags.rs"]
pub mod flags;

pub use flags::{boot_flags, fadt_flags};
