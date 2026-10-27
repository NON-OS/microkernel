// NONOS Operating System (AGPL-3.0-or-later)

//! The per-channel RX gain settings of `rtw8852bx_ctrl_ch` (Linux v6.12
//! rtw89 `rtw8852b_common.c`): `rtw8852bx_set_gain_error` (the BB gain
//! table's LNA and TIA values for this gain band) and
//! `rtw8852bx_set_gain_offset` (the efuse's corrections, clamped to S8).

use super::bb::Bb;
use super::state::Phy;
use crate::chan::Chan;
use crate::regs::Mmio;

/// `bb_gain_lna`: (2G path A, 2G path B, 5G path A, 5G path B, mask).
const LNA: [(u32, u32, u32, u32, u32); 7] = [
    (0x4678, 0x475c, 0x45dc, 0x4740, 0x00ff_0000),
    (0x4678, 0x475c, 0x45dc, 0x4740, 0xff00_0000),
    (0x467c, 0x4760, 0x4660, 0x4744, 0x0000_00ff),
    (0x467c, 0x4760, 0x4660, 0x4744, 0x0000_ff00),
    (0x467c, 0x4760, 0x4660, 0x4744, 0x00ff_0000),
    (0x467c, 0x4760, 0x4660, 0x4744, 0xff00_0000),
    (0x4680, 0x4764, 0x4664, 0x4748, 0x0000_00ff),
];
/// `bb_gain_tia`.
const TIA: [(u32, u32, u32, u32, u32); 2] =
    [(0x4680, 0x4764, 0x4664, 0x4748, 0x00ff_0000), (0x4680, 0x4764, 0x4664, 0x4748, 0xff00_0000)];

fn reg(e: &(u32, u32, u32, u32, u32), five: bool, path: usize) -> u32 {
    [[e.0, e.1], [e.2, e.3]][five as usize][path]
}

pub fn gain_error(bb: &Bb<'_, impl Mmio>, p: &Phy, c: Chan, path: usize) {
    let band = c.gain_band();
    for (i, e) in LNA.iter().enumerate() {
        bb.mask(reg(e, c.five, path), e.4, p.gain.lna[band][path][i] as i32 as u32);
    }
    for (i, e) in TIA.iter().enumerate() {
        bb.mask(reg(e, c.five, path), e.4, p.gain.tia[band][path][i] as i32 as u32);
    }
}

fn clamp8(v: i32) -> u32 {
    v.clamp(i8::MIN as i32, i8::MAX as i32) as u32
}

pub fn gain_offset(bb: &Bb<'_, impl Mmio>, p: &Phy, c: Chan) {
    let g = &p.efuse_gain;
    if g.comp_valid {
        for (path, a) in [0x4acc, 0x4ad8].into_iter().enumerate() {
            bb.mask(a, 0xff, clamp8((g.comp[path][c.subband() as usize] as i32) << 2));
        }
    }
    if !g.offset_valid {
        return;
    }
    let ob = g.offset_base as i32;
    let band = c.ofdm_offset_band();
    for (path, a) in [0x4694, 0x4778].into_iter().enumerate() {
        let off = -(g.offset[path][band] as i32);
        bb.mask(a, 0xff_0000, clamp8(-((off << 2) + (ob >> 2))));
    }
    // antenna_rx is never set to path B alone here: path A's offsets.
    let (ofdm, cck) = (-(g.offset[0][band] as i32), -(g.offset[0][0] as i32));
    bb.mask(0x49b0, 0xff, clamp8((ofdm << 4) + ob));
    bb.mask(0x4a00, 0xff, clamp8((ofdm << 4) + g.rssi_base as i32));
    if !c.five {
        let v = ((cck << 3) + (ob >> 1)).clamp(i8::MIN as i32 >> 1, i8::MAX as i32 >> 1);
        bb.mask(0x23ac, 0x7f, v as u32);
    }
}
