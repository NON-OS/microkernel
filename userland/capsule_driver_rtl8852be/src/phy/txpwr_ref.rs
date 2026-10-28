// NONOS Operating System (AGPL-3.0-or-later)

//! The BB's TX power reference, `rtw8852bx_set_txpwr_ref` with
//! `rtw8852bx_bb_cal_txpwr_ref` (Linux v6.12 rtw89 `rtw8852b_common.c`),
//! written by `rtw89_phy_dm_init` before any channel.

use super::bb::Bb;
use crate::regs::{Mmio, Rw};

const R_AX_PWR_RATE_CTRL: u32 = 0xd200;

/// `rtw8852bx_set_txpwr_ref` for PHY 0 with both references 0.
pub fn set_txpwr_ref(r: &impl Mmio) {
    r.mask32(R_AX_PWR_RATE_CTRL, 0xfff_fc00, 0);
    // rtw8852bx_bb_cal_txpwr_ref(0): TSSI offset 0x12c - 128 = 0xac, power
    // code (0x27 << 3) = 0x138 (RF 39, BB 0), reference 0.
    let v = (0xac << 18) | (0x138 << 9);
    for base in [0x5800, 0x7800] {
        Bb(r).mask(base + 4, 0x7ff_ffff, v); // OFDM
        Bb(r).mask(base + 8, 0x7ff_ffff, v); // CCK
    }
}
