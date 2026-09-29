// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/data/ioapic.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/data/ioapic.rs"]
pub mod ioapic;

pub fn ioapicinfo_gsi_max(this: ioapic::IoApicInfo) -> u32 {
    this.gsi_max()
}

