// NONOS Operating System (AGPL-3.0-or-later)

//! Into leisure power save and out of it, in rtw88's order
//! (`rtw_enter_lps_core`, `rtw_leave_lps_core`).

use super::state::Ps;
use crate::regs::Mmio;

impl Ps {
    pub(super) fn enter<M: Mmio>(&mut self, mmio: &M) {
        // The link may enter L1 only once the firmware sleeps.
        self.aspm.link_ps(mmio, true);
        self.asleep = true;
    }

    pub(super) fn leave<M: Mmio>(&mut self, mmio: &M) {
        // Out of L1 before the firmware is woken.
        self.aspm.link_ps(mmio, false);
        self.asleep = false;
    }
}
