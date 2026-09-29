// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/boot/handoff/types/framebuffer.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod boot;

pub fn framebufferinfo_is_valid(this: crate::boot::handoff::types::framebuffer::FramebufferInfo) -> bool {
    this.is_valid()
}

pub fn framebufferinfo_bytes_per_pixel(this: crate::boot::handoff::types::framebuffer::FramebufferInfo) -> u32 {
    this.bytes_per_pixel()
}

