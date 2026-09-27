// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/data/interrupt.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/data/interrupt.rs"]
pub mod interrupt;

pub fn interruptoverride_is_active_low(this: interrupt::InterruptOverride) -> bool {
    this.is_active_low()
}

pub fn interruptoverride_is_level_triggered(this: interrupt::InterruptOverride) -> bool {
    this.is_level_triggered()
}

pub fn nmiconfig_applies_to_all(this: interrupt::NmiConfig) -> bool {
    this.applies_to_all()
}

