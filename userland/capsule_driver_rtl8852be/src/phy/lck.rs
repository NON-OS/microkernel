// NONOS Operating System (AGPL-3.0-or-later)

//! `_lck_check` (Linux v6.12 rtw89 `rtw8852b_rfk.c`): when path A's
//! synthesiser reports no lock (RF 0xc5 bit 15 clear) after a channel
//! write, reset its MMD; still unlocked 10 us later, rewrite RF 0x18 with
//! the lock trigger selected; still unlocked, power-cycle the synthesiser
//! and rewrite it once more.

use super::bb::Bb;
use super::rf::RFREG_MASK;
use crate::regs::Mmio;

const RR_SYNFB: u32 = 0xc5;
const SYNFB_LK: u32 = 0x8000;
const RR_MMD: u32 = 0xd5;
const RR_LCK_TRG: u32 = 0xd3;
const RR_POW: u32 = 0xa0;
const RR_SX: u32 = 0xaf;
const RR_SYNLUT: u32 = 0xdd;
const RR_CFGCH: u32 = 0x18;

fn locked(bb: &Bb<'_, impl Mmio>) -> bool {
    bb.read_rf(0, RR_SYNFB, SYNFB_LK) != 0
}

fn rewrite<R: Mmio>(bb: &Bb<'_, R>, set18: &impl Fn(&Bb<'_, R>, u32) -> bool) {
    bb.write_rf(0, RR_LCK_TRG, 0x100, 1);
    let v = bb.read_rf(0, RR_CFGCH, RFREG_MASK);
    set18(bb, v);
    bb.write_rf(0, RR_LCK_TRG, 0x100, 0);
}

pub fn lck_check<R: Mmio>(bb: &Bb<'_, R>, set18: impl Fn(&Bb<'_, R>, u32) -> bool) {
    if !locked(bb) {
        bb.write_rf(0, RR_MMD, 0x100, 1); // RST_EN
        bb.write_rf(0, RR_MMD, 0x40, 0); // RST_SYN
        bb.write_rf(0, RR_MMD, 0x40, 1);
        bb.write_rf(0, RR_MMD, 0x100, 0);
    }
    // rtw89 waits 10 us; the clock's least step, 1 ms, is longer, and a
    // longer settle only gives the synthesiser more time to lock.
    crate::wait::delay_ms(1);
    if !locked(bb) {
        rewrite(bb, &set18);
    }
    if !locked(bb) {
        for addr in [RR_POW, RR_SX] {
            let v = bb.read_rf(0, addr, RFREG_MASK);
            bb.write_rf(0, addr, RFREG_MASK, v);
        }
        bb.write_rf(0, RR_SYNLUT, 0x10, 1);
        bb.write_rf(0, RR_POW, 0xc, 0);
        bb.write_rf(0, RR_POW, 0xc, 3);
        bb.write_rf(0, RR_SYNLUT, 0x10, 0);
        rewrite(bb, &set18);
    }
}
