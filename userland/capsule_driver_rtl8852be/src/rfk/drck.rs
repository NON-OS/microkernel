// NONOS Operating System (AGPL-3.0-or-later)

//! DACK's opening steps (Linux v6.12 rtw89 `rtw8852b_rfk.c`): `_afe_init`
//! (all PHY register sets clocked, then the AFE list) and `_drck`, the
//! D-die RC calibration: kick, wait up to 10 ms for done, latch, and force
//! the value it found.

use super::reg5::{run, List};
use crate::phy::bb::Bb;
use crate::regs::Mmio;
use crate::wait::until_us;

pub fn afe_init(r: &impl Mmio) {
    r.write32(0x8040, 0xf); // R_AX_PHYREG_SET
    run(&Bb(r), List::AfeInit);
}

/// `_drck`; false when the done bit never came (rtw89 logs and goes on).
pub fn drck(bb: &Bb<'_, impl Mmio>) -> bool {
    bb.mask(0xc0cc, 0x40, 1); // R_DRCK_V1 KICK
    let done = until_us(10_000, || bb.get(0xc0d0, 0x8) != 0); // R_DRCK_RS DONE
    bb.mask(0xc0cc, 0x40, 0);
    bb.mask(0xc094, 0x200, 1); // R_DRCK_FH LAT
    let _ = bb.read32(0xc094); // rtw89's udelay(1); see `rf::write_rf`
    bb.mask(0xc094, 0x200, 0);
    let rck_d = bb.get(0xc0d0, 0xf_8000); // B_DRCK_RS_LPS
    bb.mask(0xc0cc, 0x200, 0); // SEL
    bb.mask(0xc0cc, 0x1f, rck_d); // CV
    done
}
