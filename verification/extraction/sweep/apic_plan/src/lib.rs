// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/interrupt/apic/plan.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/interrupt/apic/plan.rs"]
pub mod plan;

pub fn base_phys(apic_base_msr: u64) -> u64 {
    plan::base_phys(apic_base_msr)
}

pub fn dest_reachable(apic_id: u32, x2apic: bool) -> bool {
    plan::dest_reachable(apic_id, x2apic)
}

pub fn x2apic_icr(dest: u32, low: u32) -> u64 {
    plan::x2apic_icr(dest, low)
}

pub fn needs_init_deassert(x2apic: bool, lapic_version: u32) -> bool {
    plan::needs_init_deassert(x2apic, lapic_version)
}

pub fn park_icr_low() -> u32 {
    plan::park_icr_low()
}

