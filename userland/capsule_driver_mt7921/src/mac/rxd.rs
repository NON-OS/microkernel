/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! A received frame's descriptor, as Linux v6.12 `mt7921_mac_fill_rx` walks
//! it: six words, the optional groups 4, 1, 2, 3 and 5 the second word
//! announces, twice the header offset's pad, the frame. Refused as Linux refuses: band 1, an A-MSDU or length error, a
//! translated header; and, where mac80211 would drop it later, a bad FCS
//! or an ICV or CCMP MIC error (`RX_FLAG_ONLY_MONITOR`). A frame the chip
//! decrypted comes with its packet number and key index (`sec`). The
//! signal is the best chain's RCPI from group 3 (`to_rssi`).

use super::sec::{judge, pn, Sec};

const G1: u32 = 1 << 11;
const G2: u32 = 1 << 12;
const G3: u32 = 1 << 13;
const G4: u32 = 1 << 14;
const G5: u32 = 1 << 15;
const BAD1: u32 = 1 << 25 | 1 << 26 | 1 << 27 | 1 << 28; // ICV, TKIP MIC, FCS, band 1
const BAD2: u32 = 1 << 13 | 1 << 23 | 1 << 24; // translated, A-MSDU error, too long

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RxFrame {
    pub start: usize,
    pub len: usize,
    pub rssi: i8,
    /// Decrypted by the chip: the packet number and key index.
    pub sec: Option<(u64, u8)>,
}

fn word(b: &[u8], i: usize) -> u32 {
    b.get(4 * i..4 * i + 4).map_or(0, |w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
}

/// The frame in receive buffer `b` and its signal over `antennas`.
pub fn parse(b: &[u8], antennas: u8) -> Option<RxFrame> {
    let (d1, d2) = (word(b, 1), word(b, 2));
    if b.len() < 24 || d1 & BAD1 != 0 || d2 & BAD2 != 0 {
        return None;
    }
    let (mut w, mut rcpi, mut g1) = (6usize, None, None); // in words
    for (bit, words) in [(G4, 4), (G1, 4), (G2, 2), (G3, 2)] {
        if d1 & bit != 0 {
            match bit {
                G3 => rcpi = Some(word(b, w + 1)),
                G1 => g1 = Some(4 * w),
                _ => {}
            }
            w += words;
            (4 * w < b.len()).then_some(())?;
        }
    }
    if d1 & G3 != 0 && d1 & G5 != 0 {
        rcpi = Some(word(b, w + 6));
        w += 18;
        (4 * w < b.len()).then_some(())?;
    }
    let start = 4 * w + 2 * (d2 >> 14 & 3) as usize;
    let len = b.len().checked_sub(start).filter(|&l| l >= 10)?;
    let sec = match judge(d1, b[start + 1] & 0x40 != 0) {
        Sec::Plain => None,
        Sec::Decrypted(key) => Some((pn(&b[g1?..]), key)),
        Sec::Refused => return None,
    };
    Some(RxFrame { start, len, rssi: rcpi.map_or(-128, |v| signal(v, antennas)), sec })
}

/// The strongest chain's signal in dBm, `-128` when none reads below zero.
fn signal(v: u32, antennas: u8) -> i8 {
    let chains =
        (0..antennas.count_ones().min(4)).map(|i| ((v >> (8 * i) & 0xff) as i32 - 220) / 2);
    chains.filter(|&s| s < 0).max().map_or(-128, |s| s.max(-128) as i8)
}
