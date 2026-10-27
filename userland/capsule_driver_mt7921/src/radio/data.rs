/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Reading the data ring (ring 2): frames for the station, and the chip's
//! transmit-free reports, which give tokens back (`mt7921_queue_rx_skb`
//! sorts the same way). An unasked MCU event that comes this way is taken
//! as one from the event rings is (Linux hands `PKT_TYPE_RX_EVENT` on any
//! ring to `mt7921_mcu_rx_event`), so a channel grant here is not lost;
//! replies the driver waits on come on the event rings.

use alloc::vec::Vec;

use super::Mt7921;
use crate::dma::layout::RX_BUF;
use crate::dma::Rx;
use crate::mac::{kind, rxd, txfree, Kind};

/// Buffers read at most per call, so a busy channel never holds a caller.
const BUDGET: usize = 64;
/// Frames kept aside while making room for a send.
pub const INBOX_MAX: usize = 32;

impl Mt7921 {
    /// The next frame on the ring, copied into `out`: its length and
    /// signal. Reports met on the way are acted on.
    pub(super) fn next_frame(&mut self, out: &mut [u8]) -> Option<(usize, i8)> {
        let mut buf = [0u8; RX_BUF];
        let antennas = self.caps.antennas;
        for _ in 0..BUDGET {
            let c = &mut self.card;
            let Rx::Frame(n) = c.dma.rx_data.take(&c.regs, &mut buf)? else { continue };
            let b = &buf[..n];
            match kind(b) {
                Kind::TxFree => txfree::tokens(b, |t| {
                    c.tokens.free(t);
                }),
                Kind::Frame => match rxd::parse(b, antennas) {
                    Some(f) if f.len <= out.len() => {
                        out[..f.len].copy_from_slice(&b[f.start..f.start + f.len]);
                        self.count.rx = self.count.rx.wrapping_add(1);
                        return Some((f.len, f.rssi));
                    }
                    _ => self.count.rx_bad = self.count.rx_bad.wrapping_add(1),
                },
                Kind::Event => c.mcu.note_unasked(b),
                Kind::Other => {}
            }
        }
        None
    }

    /// Read the ring dry into the inbox, for the tokens its reports free.
    pub(super) fn reap(&mut self) {
        let mut buf = [0u8; RX_BUF];
        while let Some((n, rssi)) = self.next_frame(&mut buf) {
            if self.inbox.len() < INBOX_MAX {
                self.inbox.push_back((Vec::from(&buf[..n]), rssi));
            }
        }
    }
}
