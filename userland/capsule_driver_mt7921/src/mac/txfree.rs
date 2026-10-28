/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The chip's transmit-free report, `PKT_TYPE_TXRX_NOTIFY` on the data
//! ring, as Linux v6.12 `mt7921_mac_tx_free` reads it: the count in bits
//! 25:16 of the first word (`MT_TX_FREE_MSDU_CNT` of the header's `ctrl`),
//! then one word each from byte 8. A word with `MT_TX_FREE_PAIR` set names
//! a station and does not count; any other carries a token in bits 30:16
//! (`MT_TX_FREE_MSDU_ID`). Every word read is inside the buffer.

const PAIR: u32 = 1 << 31;

/// Each token the report frees, in order, handed to `free`.
pub fn tokens(b: &[u8], mut free: impl FnMut(u16)) {
    let Some(w0) = word(b, 0) else { return };
    let mut count = (w0 >> 16 & 0x3ff) as usize;
    let mut i = 0usize;
    while i < count {
        let Some(info) = word(b, 8 + 4 * i) else { return };
        if info & PAIR != 0 {
            count += 1;
        } else {
            free((info >> 16 & 0x7fff) as u16);
        }
        i += 1;
    }
}

fn word(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at.checked_add(4)?)?.try_into().ok()?))
}
