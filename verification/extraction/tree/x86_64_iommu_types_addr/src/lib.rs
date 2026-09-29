// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/iommu/types/addr.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn iovirtaddr_new(addr: u64) -> crate::arch::x86_64::iommu::types::addr::IoVirtAddr {
    crate::arch::x86_64::iommu::types::addr::IoVirtAddr::new(addr)
}

pub fn iovirtaddr_as_u64(this: crate::arch::x86_64::iommu::types::addr::IoVirtAddr) -> u64 {
    this.as_u64()
}

pub fn iovirtaddr_is_page_aligned(this: crate::arch::x86_64::iommu::types::addr::IoVirtAddr) -> bool {
    this.is_page_aligned()
}

