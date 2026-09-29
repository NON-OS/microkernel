// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/frame_alloc/types/range.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn framerange_frames_remaining(this: crate::memory::frame_alloc::types::range::FrameRange) -> usize {
    this.frames_remaining()
}

pub fn framerange_is_exhausted(this: crate::memory::frame_alloc::types::range::FrameRange) -> bool {
    this.is_exhausted()
}

