// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/data/numa.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/data/numa.rs"]
pub mod numa;

pub fn numamemoryregion_end(this: numa::NumaMemoryRegion) -> u64 {
    this.end()
}

pub fn numamemoryregion_contains(this: numa::NumaMemoryRegion, addr: u64) -> bool {
    this.contains(addr)
}

