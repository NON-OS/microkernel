// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/hpet/protection.rs"]
pub mod protection;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/hpet/table.rs"]
pub mod table;

pub use protection::PageProtection;
pub use table::Hpet;
