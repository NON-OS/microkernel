// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/acpi/tables/fadt/fadt_methods.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn fadt_has_reset_register(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> bool {
    this.has_reset_register()
}

pub fn fadt_is_hw_reduced(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> bool {
    this.is_hw_reduced()
}

pub fn fadt_is_pm_timer_32bit(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> bool {
    this.is_pm_timer_32bit()
}

pub fn fadt_supports_low_power_s0(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> bool {
    this.supports_low_power_s0()
}

pub fn fadt_sci_interrupt(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u16 {
    this.sci_interrupt()
}

pub fn fadt_c2_latency_us(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u16 {
    this.c2_latency_us()
}

pub fn fadt_c3_latency_us(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u16 {
    this.c3_latency_us()
}

pub fn fadt_supports_c2(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> bool {
    this.supports_c2()
}

pub fn fadt_supports_c3(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> bool {
    this.supports_c3()
}

