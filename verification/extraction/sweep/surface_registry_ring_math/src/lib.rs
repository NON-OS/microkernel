// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/kernel_core/surface_registry/ring_math.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/kernel_core/surface_registry/ring_math.rs"]
pub mod ring_math;

pub fn wrap(pos: usize, cap: usize) -> usize {
    ring_math::wrap(pos, cap)
}

pub fn is_full(head: usize, tail: usize, cap: usize) -> bool {
    ring_math::is_full(head, tail, cap)
}

