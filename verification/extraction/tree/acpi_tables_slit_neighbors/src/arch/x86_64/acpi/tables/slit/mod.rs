// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/slit/neighbors.rs"]
pub mod neighbors;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/slit/numa.rs"]
pub mod numa;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/slit/table.rs"]
pub mod table;

pub use numa::NumaDistances;
pub use table::Slit;
