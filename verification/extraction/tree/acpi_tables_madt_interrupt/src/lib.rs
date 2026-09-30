// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/acpi/tables/madt/interrupt.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn madtinterruptoverride_polarity(this: crate::arch::x86_64::acpi::tables::madt::interrupt::MadtInterruptOverride) -> u8 {
    this.polarity()
}

pub fn madtinterruptoverride_trigger_mode(this: crate::arch::x86_64::acpi::tables::madt::interrupt::MadtInterruptOverride) -> u8 {
    this.trigger_mode()
}

pub fn madtinterruptoverride_is_active_low(this: crate::arch::x86_64::acpi::tables::madt::interrupt::MadtInterruptOverride) -> bool {
    this.is_active_low()
}

pub fn madtinterruptoverride_is_level_triggered(this: crate::arch::x86_64::acpi::tables::madt::interrupt::MadtInterruptOverride) -> bool {
    this.is_level_triggered()
}

pub fn madtinterruptoverride_is_edge_triggered(this: crate::arch::x86_64::acpi::tables::madt::interrupt::MadtInterruptOverride) -> bool {
    this.is_edge_triggered()
}

pub fn madtinterruptoverride_is_active_high(this: crate::arch::x86_64::acpi::tables::madt::interrupt::MadtInterruptOverride) -> bool {
    this.is_active_high()
}

pub fn madtnmisource_polarity(this: crate::arch::x86_64::acpi::tables::madt::interrupt::MadtNmiSource) -> u8 {
    this.polarity()
}

pub fn madtnmisource_trigger_mode(this: crate::arch::x86_64::acpi::tables::madt::interrupt::MadtNmiSource) -> u8 {
    this.trigger_mode()
}

