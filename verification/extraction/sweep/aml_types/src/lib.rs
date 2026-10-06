// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/aml/types/mod.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/aml/types/mod.rs"]
pub mod types;

pub fn lpsscontroller_is_valid(this: types::LpssController) -> bool {
    this.is_valid()
}

pub fn gpiocontroller_is_valid(this: types::GpioController) -> bool {
    this.is_valid()
}

