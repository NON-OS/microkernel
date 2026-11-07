// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/kernel_core/init/framebuffer/hidpi.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/kernel_core/init/framebuffer/hidpi.rs"]
pub mod hidpi;

pub fn scale_for(width: u32, height: u32) -> u32 {
    hidpi::scale_for(width, height)
}

