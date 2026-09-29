// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/phys/types/frame.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn frame_new(addr: u64) -> crate::memory::phys::types::frame::Frame {
    crate::memory::phys::types::frame::Frame::new(addr)
}

pub fn frame_addr(this: crate::memory::phys::types::frame::Frame) -> u64 {
    this.addr()
}

pub fn frame_number(this: crate::memory::phys::types::frame::Frame, base: u64, page_size: u64) -> u64 {
    this.number(base, page_size)
}

pub fn frame_is_null(this: crate::memory::phys::types::frame::Frame) -> bool {
    this.is_null()
}

