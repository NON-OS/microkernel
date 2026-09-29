// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/hardening/stats/record.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn hardeningstats_increment_guard_violations(this: crate::memory::hardening::stats::types::HardeningStats) {
    this.increment_guard_violations()
}

pub fn hardeningstats_increment_wx_violations(this: crate::memory::hardening::stats::types::HardeningStats) {
    this.increment_wx_violations()
}

pub fn hardeningstats_increment_stack_overflows(this: crate::memory::hardening::stats::types::HardeningStats) {
    this.increment_stack_overflows()
}

pub fn hardeningstats_increment_heap_corruptions(this: crate::memory::hardening::stats::types::HardeningStats) {
    this.increment_heap_corruptions()
}

pub fn hardeningstats_increment_double_frees(this: crate::memory::hardening::stats::types::HardeningStats) {
    this.increment_double_frees()
}

pub fn hardeningstats_increment_use_after_free(this: crate::memory::hardening::stats::types::HardeningStats) {
    this.increment_use_after_free()
}

