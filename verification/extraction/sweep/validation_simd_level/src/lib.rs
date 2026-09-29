// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/boot/validation/simd_level.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/boot/validation/simd_level.rs"]
pub mod simd_level;

pub fn simdlevel_register_width(this: simd_level::SimdLevel) -> usize {
    this.register_width()
}

