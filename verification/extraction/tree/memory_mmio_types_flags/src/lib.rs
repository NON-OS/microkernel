// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/mmio/types/flags.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn mmioflags_device() -> crate::memory::mmio::types::flags::MmioFlags {
    crate::memory::mmio::types::flags::MmioFlags::device()
}

pub fn mmioflags_framebuffer() -> crate::memory::mmio::types::flags::MmioFlags {
    crate::memory::mmio::types::flags::MmioFlags::framebuffer()
}

pub fn mmioflags_user_device() -> crate::memory::mmio::types::flags::MmioFlags {
    crate::memory::mmio::types::flags::MmioFlags::user_device()
}

pub fn mmioflags_to_vm_flags(this: crate::memory::mmio::types::flags::MmioFlags) -> u32 {
    this.to_vm_flags()
}

