// NONOS Operating System (AGPL-3.0-or-later)

//! TX power for a channel: `__rtw8852bx_set_txpwr` (Linux v6.12 rtw89
//! `rtw8852b_common.c`) under the world-wide regulation, whose register
//! images `tables/gen_txpwr.py` computes from rtw89's tables (by rate,
//! rate offsets, limits, RU limits); the TX shape for RTW89_WW is flat on
//! both CCK and OFDM. The BB's power reference is `txpwr_ref`'s.

use super::bb::Bb;
use crate::chan::Chan;
use crate::regs::{Mmio, Rw};

pub const TXPWR: &[u8] = include_bytes!("../../tables/txpwr.bin");
pub const BYRATE_WORDS: usize = 11;
pub const CH_WORDS: usize = 32;
const R_AX_PWR_RATE_OFST_CTRL: u32 = 0xd204;
const R_AX_PWR_BY_RATE: u32 = 0xd2c0;
const R_AX_PWR_LMT: u32 = 0xd2ec;
/// `param_flat` of `rtw8852bx_bb_set_tx_shape_dfir`, then channel 14's.
const DFIR: [[u32; 8]; 2] = [
    [
        0x023d23ff, 0x0029b354, 0x000fc1c8, 0x00fdb053, 0x00f86f9a, 0x06faef92, 0x00fe5fcc,
        0x00ffdff5,
    ],
    [
        0x023b13ff, 0x001c42de, 0x00fdb0ad, 0x00f60f6e, 0x00fd8f92, 0x0602d011, 0x0001c02c,
        0x00fff00a,
    ],
];

fn word(i: usize) -> u32 {
    u32::from_le_bytes([TXPWR[4 * i], TXPWR[4 * i + 1], TXPWR[4 * i + 2], TXPWR[4 * i + 3]])
}

/// `rtw89_channel_to_idx` over both bands, 2 GHz first.
pub const fn ch_index(c: Chan) -> usize {
    match (c.five, c.ch) {
        (false, ch) => ch as usize - 1,
        (true, ch @ 36..=64) => 14 + (ch as usize - 36) / 2,
        (true, ch @ 100..=144) => 14 + 15 + (ch as usize - 100) / 2,
        (true, ch) => 14 + 38 + (ch as usize - 149) / 2,
    }
}

pub fn set_txpwr(r: &impl Mmio, c: Chan) {
    let band = c.five as usize;
    for i in 0..BYRATE_WORDS {
        r.write32(R_AX_PWR_BY_RATE + 4 * i as u32, word(band * BYRATE_WORDS + i));
    }
    r.mask32(R_AX_PWR_RATE_OFST_CTRL, 0xf_ffff, word(2 * BYRATE_WORDS + band));
    if !c.five {
        let dfir = &DFIR[(c.ch == 14) as usize];
        for (i, &v) in dfir.iter().enumerate() {
            Bb(r).write32(0x2300 + 4 * i as u32, v); // R_TXFIR0..
        }
    }
    Bb(r).mask(0x4494, 0x300_0000, 0); // R_DCFO_OPT B_TXSHAPE_TRIANGULAR_CFG
    let base = 2 * BYRATE_WORDS + 2 + ch_index(c) * CH_WORDS;
    // Limits then RU limits: contiguous from R_AX_PWR_LMT to 0xd36c.
    for i in 0..CH_WORDS {
        r.write32(R_AX_PWR_LMT + 4 * i as u32, word(base + i));
    }
}
