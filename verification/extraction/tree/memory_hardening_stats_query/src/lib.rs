// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/hardening/stats/query.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn hardeningstats_guard_violations(this: crate::memory::hardening::stats::types::HardeningStats) -> u64 {
    this.guard_violations()
}

pub fn hardeningstats_wx_violations(this: crate::memory::hardening::stats::types::HardeningStats) -> u64 {
    this.wx_violations()
}

pub fn hardeningstats_stack_overflows(this: crate::memory::hardening::stats::types::HardeningStats) -> u64 {
    this.stack_overflows()
}

pub fn hardeningstats_heap_corruptions(this: crate::memory::hardening::stats::types::HardeningStats) -> u64 {
    this.heap_corruptions()
}

pub fn hardeningstats_double_frees(this: crate::memory::hardening::stats::types::HardeningStats) -> u64 {
    this.double_frees()
}

pub fn hardeningstats_use_after_free(this: crate::memory::hardening::stats::types::HardeningStats) -> u64 {
    this.use_after_free()
}

