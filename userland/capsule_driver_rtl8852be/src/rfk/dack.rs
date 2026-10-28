// NONOS Operating System (AGPL-3.0-or-later)

//! DAC calibration: `_dack_s0`, `_dack_s1` and `_dac_cal` (Linux v6.12
//! rtw89 `rtw8852b_rfk.c`). Each path runs its MSB calibration list, waits
//! up to 10 ms for done, runs the DC list, waits again, closes, and reads
//! back what it found. rtw89's done checks differ by path and are kept:
//! path A needs both words' done bits, path B either one. Only path B runs
//! `_check_dadc`, as in rtw89.

use super::addck::{addck, addck_backup_reload, R_DBG};
use super::dack_read::{backup, s0_done, s1_done};
use super::drck::{afe_init, drck};
use super::rck::{RR_MOD, RR_RSV1, RR_RSV1_RST};
use super::reg5::{run, List};
use super::DackInfo;
use crate::phy::bb::Bb;
use crate::phy::rf::RFREG_MASK;
use crate::regs::Mmio;
use crate::wait::until_us;

fn dack_path(bb: &Bb<'_, impl Mmio>, d: &mut DackInfo, path: usize) {
    let lists = [
        [List::DackS0_1, List::DackS0_2, List::DackS0_3],
        [List::DackS1_1, List::DackS1_2, List::DackS1_3],
    ][path];
    let done = |p1| if path == 0 { s0_done(bb, p1) } else { s1_done(bb, p1) };
    run(bb, lists[0]);
    d.msbk_timeout[path] = !until_us(10_000, || done(true));
    run(bb, lists[1]);
    d.dadck_timeout[path] = !until_us(10_000, || done(false));
    run(bb, lists[2]);
    if path == 1 {
        run(bb, List::CheckDadcEnB);
        run(bb, List::CheckAddcB);
        for _ in 0..100 {
            let _ = bb.read32(0x1730);
        }
        run(bb, List::CheckDadcDisB);
    }
    backup(bb, d, path);
    bb.mask(R_DBG[path], 0x4000_0000, 0);
}

/// `_dac_cal`; `drck_timeout` records the D-die RCK's.
pub fn dac_cal(r: &impl Mmio, d: &mut DackInfo) {
    let bb = Bb(r);
    let rf0 = [bb.read_rf(0, RR_MOD, RFREG_MASK), bb.read_rf(1, RR_MOD, RFREG_MASK)];
    afe_init(r);
    d.drck_timeout = !drck(&bb);
    for path in 0..2u8 {
        bb.write_rf(path, RR_RSV1, RR_RSV1_RST, 0);
    }
    for path in 0..2u8 {
        bb.write_rf(path, RR_MOD, RFREG_MASK, 0x337e1);
    }
    addck(&bb, d);
    addck_backup_reload(&bb, d);
    for path in 0..2u8 {
        bb.write_rf(path, 0x1, RFREG_MASK, 0); // RR_MODOPT
    }
    dack_path(&bb, d, 0);
    dack_path(&bb, d, 1);
    d.done = true;
    for path in 0..2u8 {
        bb.write_rf(path, RR_MOD, RFREG_MASK, rf0[path as usize]);
    }
    for path in 0..2u8 {
        bb.write_rf(path, RR_RSV1, RR_RSV1_RST, 1);
    }
}
