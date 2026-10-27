// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8852bx_set_rxsc_rpl_comp` (Linux v6.12 rtw89 `rtw8852b_common.c`):
//! the BB gain table's RPL offsets for this gain band, averaged over the
//! two paths, into both paths' RPL registers.

use super::bb::Bb;
use super::state::Phy;
use crate::chan::Chan;
use crate::regs::Mmio;

fn avg(a: i8, b: i8) -> u32 {
    ((a as i32 + b as i32) >> 1) as u32
}

fn pack(v: [u32; 4]) -> u32 {
    (v[0] & 0xff) | (v[1] & 0xff) << 8 | (v[2] & 0xff) << 16 | (v[3] & 0xff) << 24
}

pub fn rpl_comp(bb: &Bb<'_, impl Mmio>, p: &Phy, c: Chan) {
    let b = c.gain_band();
    let g = &p.gain;
    let r40 = |i: usize| avg(g.rpl_40[b][0][i], g.rpl_40[b][1][i]);
    let r80 = |i: usize| avg(g.rpl_80[b][0][i], g.rpl_80[b][1][i]);
    // B_P0_RPL1_20/40/41 sit at bits 8..32; the value is shifted down by 8
    // and written under 0xffffff00, which puts it back.
    let v1 = pack([0, avg(g.rpl_20[b][0], g.rpl_20[b][1]), r40(0), r40(1)]) >> 8;
    bb.mask(0x49b0, 0xffff_ff00, v1); // R_P0_RPL1
    bb.mask(0x4a00, 0xffff_ff00, v1); // R_P1_RPL1
    let v2 = pack([r40(2), r80(0), r80(1), r80(10)]);
    bb.write32(0x49b4, v2);
    bb.write32(0x4a04, v2);
    let v3 = pack([r80(2), r80(3), r80(4), r80(9)]);
    bb.write32(0x49b8, v3);
    bb.write32(0x4a08, v3);
}
