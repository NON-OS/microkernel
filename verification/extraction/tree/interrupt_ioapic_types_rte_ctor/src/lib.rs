// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/interrupt/ioapic/types_rte/ctor.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn rte_fixed(vector: u8, dest_apic_id: u32) -> crate::arch::x86_64::interrupt::ioapic::types_rte::types::Rte {
    crate::arch::x86_64::interrupt::ioapic::types_rte::types::Rte::fixed(vector, dest_apic_id)
}

pub fn rte_nmi(dest_apic_id: u32) -> crate::arch::x86_64::interrupt::ioapic::types_rte::types::Rte {
    crate::arch::x86_64::interrupt::ioapic::types_rte::types::Rte::nmi(dest_apic_id)
}

