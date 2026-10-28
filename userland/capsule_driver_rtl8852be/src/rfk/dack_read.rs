// NONOS Operating System (AGPL-3.0-or-later)

//! DACK's done checks and read-back (Linux v6.12 rtw89 `rtw8852b_rfk.c`):
//! `_dack_s0_check_done` (both words' bits), `_dack_s1_check_done` (either
//! word's), and `_dack_backup_s0`/`_s1`: the sixteen MSB calibration
//! values per I and Q, the bias and the DADCK values.

use super::addck::R_DBG;
use super::DackInfo;
use crate::phy::bb::Bb;
use crate::regs::Mmio;

pub fn s0_done(bb: &Bb<'_, impl Mmio>, part1: bool) -> bool {
    let (a, b, m) = if part1 { (0xc040, 0xc064, 1 << 31) } else { (0xc05c, 0xc080, 0x4) };
    bb.get(a, m) != 0 && bb.get(b, m) != 0
}

pub fn s1_done(bb: &Bb<'_, impl Mmio>, part1: bool) -> bool {
    let (a, b, m) = if part1 { (0xc140, 0xc164, 1 << 31) } else { (0xc15c, 0xc180, 0x4) };
    bb.get(a, m) != 0 || bb.get(b, m) != 0
}

/// (MSBK select, MSBK read) for I then Q, then bias and DADCK registers.
const BACKUP: [[(u32, u32); 4]; 2] = [
    [(0xc000, 0xc05c), (0xc020, 0xc080), (0xc048, 0xc06c), (0xc060, 0xc084)],
    [(0xc100, 0xc15c), (0xc120, 0xc180), (0xc148, 0xc16c), (0xc160, 0xc184)],
];

pub fn backup(bb: &Bb<'_, impl Mmio>, d: &mut DackInfo, path: usize) {
    let t = &BACKUP[path];
    bb.mask(R_DBG[path], 0x4000_0000, 1);
    for i in 0..16 {
        bb.mask(t[0].0, 0x1e, i as u32);
        d.msbk[path][0][i] = bb.get(t[0].1, 0xff00_0000) as u8;
        bb.mask(t[1].0, 0x1e, i as u32);
        d.msbk[path][1][i] = bb.get(t[1].1, 0xff00_0000) as u8;
    }
    d.biask[path] = [bb.get(t[2].0, 0xffc), bb.get(t[2].1, 0xffc)];
    d.dadck[path] = [bb.get(t[3].0, 0xff00_0000), bb.get(t[3].1, 0xff00_0000)];
}
