// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/fadt/fadt_methods.rs"]
pub mod fadt_methods;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/fadt/fadt_struct.rs"]
pub mod fadt_struct;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/fadt/flags.rs"]
pub mod flags;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/fadt/profile.rs"]
pub mod profile;

pub use flags::{boot_flags, fadt_flags};
pub use profile::PmProfile;
