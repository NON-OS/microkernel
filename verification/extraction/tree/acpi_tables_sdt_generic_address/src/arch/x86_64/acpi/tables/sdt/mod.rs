// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/sdt/address_space.rs"]
pub mod address_space;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/sdt/generic_address.rs"]
pub mod generic_address;

pub use address_space::AddressSpace;
pub use generic_address::GenericAddress;
