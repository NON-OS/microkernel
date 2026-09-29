// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/cpu/features/types_new.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn cpufeatures_new() -> crate::arch::x86_64::cpu::features::types_struct::CpuFeatures {
    crate::arch::x86_64::cpu::features::types_struct::CpuFeatures::new()
}

