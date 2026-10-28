// NONOS Operating System (AGPL-3.0-or-later)

//! The fixed rates `rtw89_core_get_mgmt_rate` and `rtw89_core_get_data_rate`
//! (Linux v6.12 rtw89 `core.c`) choose: the band's lowest (CCK 1 Mb/s on
//! 2.4 GHz, OFDM 6 Mb/s on 5 GHz), or once an AP is chosen the lowest of
//! its basic rates (management) or of its supported rates (a data frame's
//! lowest retry rate). mac80211 learns the basic rates from the AP's
//! beacon; this driver reads them from the beacons it receives of the BSS
//! it is joining (`learn`).

/// mac80211's bit rates per band, in 500 kb/s units; a rate's index plus
/// the band's lowest hardware rate is its hardware rate code.
pub const RATES_2G: [u8; 12] = [2, 4, 11, 22, 12, 18, 24, 36, 48, 72, 96, 108];
pub const RATES_5G: [u8; 8] = [12, 18, 24, 36, 48, 72, 96, 108];
pub const CCK1: u16 = 0x0;
pub const OFDM6: u16 = 0x4;

pub const fn lowest(five: bool) -> u16 {
    if five {
        OFDM6
    } else {
        CCK1
    }
}

/// The band's bitmaps of `rates` (supported, basic) as mac80211 keeps them.
pub fn bitmaps(rates: &[u8], five: bool) -> (u32, u32) {
    let table: &[u8] = if five { &RATES_5G } else { &RATES_2G };
    rates.iter().fold((0, 0), |(s, b), r| match table.iter().position(|t| *t == r & 0x7f) {
        Some(i) if r & 0x80 != 0 => (s | 1 << i, b | 1 << i),
        Some(i) => (s | 1 << i, b),
        None => (s, b),
    })
}

/// `__ffs(bitmap) + lowest`, or the lowest rate for an empty bitmap.
pub fn pick(bitmap: u32, five: bool) -> u16 {
    match bitmap {
        0 => lowest(five),
        m => m.trailing_zeros() as u16 + lowest(five),
    }
}

/// The Supported Rates (1) and Extended Supported Rates (50) of a beacon
/// or probe response body's elements, up to 16.
pub fn learn(ies: &[u8], out: &mut [u8; 16]) -> usize {
    let (mut n, mut i) = (0, 0);
    while i + 2 <= ies.len() {
        let (id, len) = (ies[i], ies[i + 1] as usize);
        let Some(v) = ies.get(i + 2..i + 2 + len) else { break };
        if id == 1 || id == 50 {
            for &r in v.iter().take(16 - n) {
                out[n] = r;
                n += 1;
            }
        }
        i += 2 + len;
    }
    n
}
