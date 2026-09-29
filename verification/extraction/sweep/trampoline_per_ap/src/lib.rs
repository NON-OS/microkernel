// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/smp/trampoline/per_ap.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/smp/trampoline/per_ap.rs"]
pub mod per_ap;

pub fn perapbootcontext_new(pml4_phys: u64, stack_top: u64, entry_ptr: u64, cpu_id: u32) -> per_ap::PerApBootContext {
    per_ap::PerApBootContext::new(pml4_phys, stack_top, entry_ptr, cpu_id)
}

