// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/layout/types/percpu.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/layout/types/percpu.rs"]
pub mod percpu;

pub fn percpuregion_new(base: u64, size: usize, cpu_id: u32) -> percpu::PercpuRegion {
    percpu::PercpuRegion::new(base, size, cpu_id)
}

pub fn percpuregion_end(this: percpu::PercpuRegion) -> u64 {
    this.end()
}

pub fn percpuregion_contains(this: percpu::PercpuRegion, addr: u64) -> bool {
    this.contains(addr)
}

