/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The frame path's formats: what goes with a frame onto ring 0, what
//! comes with one off ring 2, and the chip's reports in between.

pub mod rate;
pub mod rxd;
pub mod tokens;
pub mod txd;
pub mod txfree;

/// What a data-ring buffer is, by `RXD0`'s packet type (bits 31:27) and
/// flag (19:16), as `mt7921_queue_rx_skb` sorts it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A received frame (`PKT_TYPE_NORMAL`, or one the MCU passes on).
    Frame,
    /// A transmit-free report (`PKT_TYPE_TXRX_NOTIFY`).
    TxFree,
    /// An MCU event that came on the data ring.
    Event,
    /// Anything else: transmit status, and what this driver does not read.
    Other,
}

pub fn kind(b: &[u8]) -> Kind {
    let Some(w) = b.get(..4) else { return Kind::Other };
    let dw0 = u32::from_le_bytes([w[0], w[1], w[2], w[3]]);
    match (dw0 >> 27, dw0 >> 16 & 0xf) {
        (2, _) | (8, _) | (7, 1) => Kind::Frame,
        (6, _) => Kind::TxFree,
        (7, _) => Kind::Event,
        _ => Kind::Other,
    }
}
