/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Legacy rates: the TXD's fixed-rate value and mac80211's rate bitmaps,
//! from Linux v6.12 `mt76/mac80211.c`'s `mt76_rates` (CCK index 0 to 3 at
//! long preamble, OFDM by the chip's own index) and
//! `mt76_connac2_mac_tx_rate_val` (`MT_TX_RATE_MODE` at bits 9:6, the index
//! at 5:0). 2.4 GHz counts the four CCK rates then the eight OFDM; 5 GHz
//! the eight OFDM alone (`mt76_calculate_default_rate`'s offset of four).

/// Each legacy rate, in 500 kb/s units, as `mt76_rates` orders them, with
/// its mode (0 CCK, 1 OFDM) and the chip's index for it.
const RATES: [(u8, u16, u16); 12] = [
    (2, 0, 0),
    (4, 0, 1),
    (11, 0, 2),
    (22, 0, 3),
    (12, 1, 11),
    (18, 1, 15),
    (24, 1, 10),
    (36, 1, 14),
    (48, 1, 9),
    (72, 1, 13),
    (96, 1, 8),
    (108, 1, 12),
];

fn table(ghz5: bool) -> &'static [(u8, u16, u16)] {
    if ghz5 {
        &RATES[4..]
    } else {
        &RATES
    }
}

/// The TXD6 `TX_RATE` value for `rate` (500 kb/s units), or the band's
/// lowest when the band has no such rate.
pub fn value(rate: u8, ghz5: bool) -> u16 {
    let t = table(ghz5);
    let (_, mode, idx) = t.iter().find(|r| r.0 == rate & 0x7f).copied().unwrap_or(t[0]);
    mode << 6 | idx
}

/// The band's lowest rate: what management frames go at.
pub fn lowest(ghz5: bool) -> u8 {
    table(ghz5)[0].0
}

/// mac80211's bitmaps for `rates` (Supported Rates entries): every rate
/// named, and those marked basic (bit 7).
pub fn bitmaps(rates: &[u8], ghz5: bool) -> (u16, u16) {
    let (mut supp, mut basic) = (0u16, 0u16);
    for (i, r) in table(ghz5).iter().enumerate() {
        if let Some(&named) = rates.iter().find(|&&x| x & 0x7f == r.0) {
            supp |= 1 << i;
            basic |= u16::from(named & 0x80 != 0) << i;
        }
    }
    (supp, basic)
}

/// The fastest basic rate in `rates`, or the band's lowest: what data goes
/// at, as the Intel driver sends it, with no rate scaling.
pub fn data_rate(rates: &[u8], ghz5: bool) -> u8 {
    let basic = rates.iter().filter(|&&r| r & 0x80 != 0).map(|&r| r & 0x7f);
    basic.filter(|&r| table(ghz5).iter().any(|t| t.0 == r)).max().unwrap_or(lowest(ghz5))
}
