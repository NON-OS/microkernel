// NONOS Operating System (AGPL-3.0-or-later)

//! The 8852B's BB reset, `rtw8852b_bb_reset` around
//! `__rtw8852bx_bb_reset_all` (Linux v6.12 rtw89 `rtw8852b.c`,
//! `rtw8852b_common.c`): TX power and TSSI tracking held while the BB's
//! serial interfaces are stopped and its async reset is pulsed.

use super::bb::Bb;
use crate::regs::Mmio;
use crate::wait::delay_ms;

/// `rtw8852b_bb_reset`: TX power and TSSI tracking held around
/// `__rtw8852bx_bb_reset_all`.
pub fn bb_reset(r: &impl Mmio) {
    let bb = Bb(r);
    let held = [(0x58dc, 1 << 30), (0x5818, 1 << 30), (0x78dc, 1 << 30), (0x7818, 1 << 30)];
    for (a, b) in held {
        bb.set(a, b);
    }
    reset_all(r);
    for (a, b) in held {
        bb.clr(a, b);
    }
}

/// `__rtw8852bx_bb_reset_all`.
pub fn reset_all(r: &impl Mmio) {
    let bb = Bb(r);
    bb.mask(0x1200, 0x7000_0000, 0x7);
    bb.mask(0x3200, 0x7000_0000, 0x7);
    delay_ms(1);
    bb.mask(0x704, 0x2, 1);
    bb.mask(0x704, 0x2, 0);
    bb.mask(0x1200, 0x7000_0000, 0x0);
    bb.mask(0x3200, 0x7000_0000, 0x0);
    bb.mask(0x704, 0x2, 1);
}
