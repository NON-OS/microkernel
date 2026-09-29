// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/acpi/tables/sdt/generic_address.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn genericaddress_empty() -> crate::arch::x86_64::acpi::tables::sdt::generic_address::GenericAddress {
    crate::arch::x86_64::acpi::tables::sdt::generic_address::GenericAddress::empty()
}

pub fn genericaddress_is_valid(this: crate::arch::x86_64::acpi::tables::sdt::generic_address::GenericAddress) -> bool {
    this.is_valid()
}

pub fn genericaddress_is_memory(this: crate::arch::x86_64::acpi::tables::sdt::generic_address::GenericAddress) -> bool {
    this.is_memory()
}

pub fn genericaddress_is_io(this: crate::arch::x86_64::acpi::tables::sdt::generic_address::GenericAddress) -> bool {
    this.is_io()
}

pub fn genericaddress_access_bytes(this: crate::arch::x86_64::acpi::tables::sdt::generic_address::GenericAddress) -> usize {
    this.access_bytes()
}

