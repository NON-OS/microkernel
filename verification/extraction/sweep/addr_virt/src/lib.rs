// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/addr/virt.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/addr/virt.rs"]
pub mod virt;

pub fn virtaddr_new(addr: u64) -> virt::VirtAddr {
    virt::VirtAddr::new(addr)
}

pub fn virtaddr_zero() -> virt::VirtAddr {
    virt::VirtAddr::zero()
}

pub fn virtaddr_as_u64(this: virt::VirtAddr) -> u64 {
    this.as_u64()
}

pub fn virtaddr_as_usize(this: virt::VirtAddr) -> usize {
    this.as_usize()
}

pub fn virtaddr_is_null(this: virt::VirtAddr) -> bool {
    this.is_null()
}

pub fn virtaddr_is_aligned(this: virt::VirtAddr, align: u64) -> bool {
    this.is_aligned(align)
}

pub fn virtaddr_align_down(this: virt::VirtAddr, align: u64) -> virt::VirtAddr {
    this.align_down(align)
}

pub fn virtaddr_align_up(this: virt::VirtAddr, align: u64) -> virt::VirtAddr {
    this.align_up(align)
}

