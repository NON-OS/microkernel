// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/census/buf.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/census/buf.rs"]
pub mod buf;

pub fn linebuf_new() -> buf::LineBuf {
    buf::LineBuf::new()
}

