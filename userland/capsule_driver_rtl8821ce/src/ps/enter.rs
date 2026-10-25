// NONOS Operating System (AGPL-3.0-or-later)

//! Into leisure power save and out of it, in rtw88's order
//! (`rtw_enter_lps_core`, `rtw_leave_lps_core`).

use super::regs::{LEAVE_MS, PWRMGT_HWDATA_EN, REG_TCR, REG_WIFI_BT_INFO, SCBD_ASLEEP, SCBD_AWAKE};
use super::state::Ps;
use crate::h2c::cmds::pwr_mode;
use crate::regs::Mmio;
use crate::wait::until_ms;

impl Ps {
    /// Bluetooth told first, then the firmware; the link may enter L1 only
    /// once the firmware sleeps. A firmware that never takes the command
    /// leaves the station awake and Bluetooth told so.
    pub(super) fn enter<M: Mmio>(&mut self, mmio: &M) {
        mmio.write16(REG_WIFI_BT_INFO, SCBD_ASLEEP);
        let (w0, w1) = pwr_mode(true);
        if !self.h2c.send(mmio, w0, w1) {
            mmio.write16(REG_WIFI_BT_INFO, SCBD_AWAKE);
            return;
        }
        self.aspm.link_ps(mmio, true);
        self.asleep = true;
    }

    /// Out of L1 first, then the firmware woken and given its time to tell
    /// the access point; frames sent before then would still carry the
    /// power management bit, so a firmware that never clears it has it
    /// cleared here, as rtw88 does. Then Bluetooth is told.
    pub(super) fn leave<M: Mmio>(&mut self, mmio: &M) {
        self.aspm.link_ps(mmio, false);
        let (w0, w1) = pwr_mode(false);
        let _ = self.h2c.send(mmio, w0, w1);
        if !until_ms(LEAVE_MS, || mmio.read32(REG_TCR) & PWRMGT_HWDATA_EN == 0) {
            let tcr = mmio.read32(REG_TCR);
            mmio.write32(REG_TCR, tcr & !PWRMGT_HWDATA_EN);
        }
        mmio.write16(REG_WIFI_BT_INFO, SCBD_AWAKE);
        self.asleep = false;
    }
}
