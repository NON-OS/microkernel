// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/addr/phys.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/addr/phys.rs"]
pub mod phys;

pub fn physaddr_new(addr: u64) -> phys::PhysAddr {
    phys::PhysAddr::new(addr)
}

pub fn physaddr_zero() -> phys::PhysAddr {
    phys::PhysAddr::zero()
}

pub fn physaddr_as_u64(this: phys::PhysAddr) -> u64 {
    this.as_u64()
}

pub fn physaddr_as_usize(this: phys::PhysAddr) -> usize {
    this.as_usize()
}

pub fn physaddr_is_null(this: phys::PhysAddr) -> bool {
    this.is_null()
}

pub fn physaddr_is_aligned(this: phys::PhysAddr, align: u64) -> bool {
    this.is_aligned(align)
}

pub fn physaddr_align_down(this: phys::PhysAddr, align: u64) -> phys::PhysAddr {
    this.align_down(align)
}

pub fn physaddr_align_up(this: phys::PhysAddr, align: u64) -> phys::PhysAddr {
    this.align_up(align)
}

