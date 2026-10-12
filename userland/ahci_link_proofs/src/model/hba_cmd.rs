// NONOS Operating System (AGPL-3.0-or-later)
//! The writes that do more than store: the command and FIS-receive
//! engines, Command List Override, and COMRESET.

use std::time::Duration;

use super::hba::Hba;
use super::quirks::CrStuck;
use super::state::LINK_UP;
use crate::constants::regs::{
    CMD_CR, CMD_FR, CMD_FRE, CMD_ST, SCTL_DET_COMRESET, SCTL_DET_MASK, TFD_BSY, TFD_DRQ,
};

/// PxCMD.CLO (AHCI 1.3.1, 3.3.7), which the driver never sets.
const CMD_CLO: u32 = 1 << 3;

impl Hba {
    pub(super) fn write_cmd(&mut self, v: u32) {
        let was = self.reg.cmd;
        if v & CMD_CLO != 0 {
            // With CAP.SCLO, the HBA's copy of BSY and DRQ only.
            self.count.clo += 1;
            self.reg.tfd &= if self.quirks.sclo { !(TFD_BSY | TFD_DRQ) } else { !0 };
        }
        let mut cmd = v & !(CMD_CLO | CMD_CR | CMD_FR) | was & CMD_CR;
        if was & CMD_ST != 0 && v & CMD_ST == 0 {
            // Clearing ST clears PxCI and PxSACT (AHCI 1.3.1, 3.3.14).
            if self.reg.ci & 1 != 0 {
                let ci_at = self.at.ci.unwrap_or_else(crate::clock::now);
                self.count.given.push(crate::clock::now() - ci_at);
            }
            (self.reg.ci, self.reg.sact, self.sent) = (0, 0, false);
            if self.quirks.cr_stuck == CrStuck::No {
                cmd &= !CMD_CR;
            }
        }
        if v & CMD_ST != 0 && was & CMD_ST == 0 {
            self.count.started_with.push((self.reg.serr, self.reg.is_read()));
            cmd |= CMD_CR;
        }
        self.reg.cmd = if v & CMD_FRE != 0 { cmd | CMD_FR } else { cmd & !CMD_FR };
    }

    pub(super) fn write_sctl(&mut self, v: u32) {
        let (was, det) = (self.reg.sctl & SCTL_DET_MASK, v & SCTL_DET_MASK);
        self.reg.sctl = v;
        let now = crate::clock::now();
        if det == SCTL_DET_COMRESET && was != SCTL_DET_COMRESET {
            self.at.reset_from = Some(now);
            self.reg.ssts = 0;
        } else if det == 0 && was == SCTL_DET_COMRESET {
            let held = self.at.reset_from.map_or(Duration::ZERO, |t| now - t);
            if held < Duration::from_millis(1) {
                // Too short for the PHY to send COMRESET (AHCI 1.3.1, 3.3.11).
                self.count.short_holds += 1;
                self.reg.ssts = LINK_UP;
                return;
            }
            // The drive answers the reset at once (DET 1); the link takes
            // longer, or never comes.
            self.count.comresets += 1;
            self.drive.reset();
            (self.sent, self.reg.tfd, self.reg.ssts) = (false, 0x80, 0x1);
            self.at.link = Some(now + Duration::from_millis(self.quirks.link_ms));
            if let Some(det) = self.quirks.stuck_det {
                (self.reg.ssts, self.at.link) = (det, None);
            }
            if self.quirks.cr_stuck == CrStuck::UntilReset {
                self.quirks.cr_stuck = CrStuck::No;
                self.reg.cmd &= !CMD_CR;
            }
        }
    }
}
