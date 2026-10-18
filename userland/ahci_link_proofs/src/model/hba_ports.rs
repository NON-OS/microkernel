// NONOS Operating System (AGPL-3.0-or-later)
//! The ports of a whole model HBA: port 0 is the `Hba` itself, ports 1 and
//! up live in its `Global`. Each answers its own register block and runs its
//! own drive, with the HBA's address width; GHC.HR resets every one of them.

use super::hba::Hba;
use super::quirks::CrStuck;
use crate::constants::regs::{CAP_S64A, CMD_CR};

impl Hba {
    /// Give every port the HBA's CAP.S64A as it reads now, before an access.
    pub(super) fn pass_width(&mut self) {
        let Some(g) = self.global.as_mut() else { return };
        let wide = g.cap & CAP_S64A != 0;
        g.extra.iter_mut().for_each(|p| p.wide = wide);
        self.wide = wide;
    }

    /// Port `i`, 1 and up, if the HBA has it.
    pub(super) fn extra(&mut self, i: u32) -> Option<&mut Hba> {
        self.global.as_mut()?.extra.get_mut(i.checked_sub(1)? as usize)
    }

    /// GHC.HR ended: every port's PxCMD.ST, CR, FRE and FR, its PxCI, PxSACT
    /// and PxIS read 0 (AHCI 1.3.1, 10.4.3). CR stays on a port whose CR
    /// never clears; all stay on a port the reset spares.
    pub(super) fn hr_reset_ports(&mut self) {
        self.hr_reset();
        self.global.iter_mut().flat_map(|g| g.extra.iter_mut()).for_each(Hba::hr_reset);
    }

    fn hr_reset(&mut self) {
        if self.quirks.hr_spares_port {
            return;
        }
        let cr = if self.quirks.cr_stuck == CrStuck::Always { CMD_CR } else { 0 };
        (self.reg.cmd, self.reg.ci, self.reg.sact, self.reg.is) = (cr, 0, 0, 0);
        self.sent = false;
    }
}
