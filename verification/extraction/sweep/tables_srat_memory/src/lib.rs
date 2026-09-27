// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/tables/srat_memory.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/tables/srat_memory.rs"]
pub mod srat_memory;

pub fn sratmemoryaffinity_is_enabled(this: srat_memory::SratMemoryAffinity) -> bool {
    this.is_enabled()
}

pub fn sratmemoryaffinity_is_hot_pluggable(this: srat_memory::SratMemoryAffinity) -> bool {
    this.is_hot_pluggable()
}

pub fn sratmemoryaffinity_is_non_volatile(this: srat_memory::SratMemoryAffinity) -> bool {
    this.is_non_volatile()
}

pub fn sratmemoryaffinity_end_address(this: srat_memory::SratMemoryAffinity) -> u64 {
    this.end_address()
}

pub fn sratmemoryaffinity_contains_address(this: srat_memory::SratMemoryAffinity, addr: u64) -> bool {
    this.contains_address(addr)
}

