// NONOS Operating System (AGPL-3.0-or-later)

//! The short calibrations of Linux v6.12 rtw89 `rtw8852b_rfk.c`: `_rck`
//! (each path's RC calibration: trigger, wait up to 30 us for done, write
//! the result back), `_rx_dck` (RX DC offset, both paths; TSSI is never
//! enabled here, so its tracking needs no hold), `_set_dpd_backoff` (all
//! `rtw8852b_dpk_init` does) and `_wait_rx_mode`.

use crate::phy::bb::Bb;
use crate::phy::rf::RFREG_MASK;
use crate::regs::Mmio;
use crate::wait::{delay_ms, until_us};

pub const RR_MOD: u32 = 0x0;
pub const RR_MOD_MASK: u32 = 0xf_0000;
pub const RR_MOD_V_RX: u32 = 0x3;
pub const RR_RSV1: u32 = 0x5;
pub const RR_RSV1_RST: u32 = 0x1;
const RR_RCKC: u32 = 0x1b;
const RR_RCKS: u32 = 0x1c;
const RR_RCKC_CA: u32 = 0x7c00;
const RR_DCK: u32 = 0x92;
const RR_DCK1: u32 = 0x93;

pub fn rck(bb: &Bb<'_, impl Mmio>, path: u8) {
    let reg5 = bb.read_rf(path, RR_RSV1, RFREG_MASK);
    bb.write_rf(path, RR_RSV1, RR_RSV1_RST, 0);
    bb.write_rf(path, RR_MOD, RR_MOD_MASK, RR_MOD_V_RX);
    bb.write_rf(path, RR_RCKC, RFREG_MASK, 0x240);
    // rtw89 reads the result whether or not the done bit came.
    let _ = until_us(30, || bb.read_rf(path, RR_RCKS, 1 << 3) != 0);
    let val = bb.read_rf(path, RR_RCKC, RR_RCKC_CA);
    bb.write_rf(path, RR_RCKC, RFREG_MASK, val);
    bb.write_rf(path, RR_RSV1, RFREG_MASK, reg5);
}

pub fn rx_dck(bb: &Bb<'_, impl Mmio>) {
    for path in 0..2u8 {
        let reg5 = bb.read_rf(path, RR_RSV1, RFREG_MASK);
        let tune = bb.read_rf(path, RR_DCK, 0x2); // RR_DCK_FINE
        bb.write_rf(path, RR_RSV1, RR_RSV1_RST, 0);
        bb.write_rf(path, RR_DCK, 0x2, 0);
        bb.write_rf(path, RR_MOD, RR_MOD_MASK, RR_MOD_V_RX);
        // _set_rx_dck
        bb.write_rf(path, RR_DCK1, 0xf, 0); // RR_DCK1_CLR
        bb.write_rf(path, RR_DCK, 0x1, 0); // RR_DCK_LV
        bb.write_rf(path, RR_DCK, 0x1, 1);
        delay_ms(1);
        bb.write_rf(path, RR_DCK, 0x2, tune);
        bb.write_rf(path, RR_RSV1, RFREG_MASK, reg5);
    }
}

/// `_set_dpd_backoff` for PHY 0: when the OFDM backoff and TX scale add
/// to 44 or more, the DPD backoff goes to the BB and is set to 0 dB.
pub fn dpd_backoff(bb: &Bb<'_, impl Mmio>) {
    let ofdm = bb.get(0x44a0, 0x1_f000); // R_DPD_BF B_DPD_BF_OFDM
    let scale = bb.get(0x44a0, 0x7f); // B_DPD_BF_SCA
    if ofdm + scale >= 44 {
        for path in 0..2u32 {
            bb.mask(0x81bc + (path << 8), 0x7f_ffff, 0x7f7f7f); // R_DPD_CH0A
        }
    }
}

/// `_wait_rx_mode`: up to 5 ms per path for RF mode to leave TX (2).
pub fn wait_rx_mode(bb: &Bb<'_, impl Mmio>) {
    for path in 0..2u8 {
        let _ = until_us(5000, || bb.read_rf(path, RR_MOD, RR_MOD_MASK) != 2);
    }
}
