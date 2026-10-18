// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! How the fake controller answers the admin commands it fetched.

use super::controller::Fake;
use super::fresh::unwritten;
use super::ring::Ring;
use crate::admin::Submission;

/// Invalid Field in Command (SCT 0, SC 02h), as a status field.
const INVALID_FIELD: u16 = 0x02 << 1;

impl Fake {
    /// Take an admin command, to be answered once due unless the queue is
    /// mute. A Create I/O CQ's ring is checked as the controller takes it.
    pub(super) fn ask(&mut self, cmd: Submission) {
        if cmd.cdw0 & 0xff == 0x05 {
            unwritten(cmd.prp1, (cmd.cdw10 >> 16) as u16 + 1, "the I/O completion ring");
        }
        self.admin_seen.push(cmd);
        if !self.mute {
            self.asked.push((cmd, self.ticks + self.hold));
        }
    }

    /// Answer every admin command now due.
    pub(super) fn answer_due(&mut self) {
        let ticks = self.ticks;
        let (due, held): (Vec<_>, _) = self.asked.iter().partition(|&&(_, at)| at <= ticks);
        self.asked = held;
        due.into_iter().for_each(|(cmd, _)| self.answer(cmd));
    }

    /// Create I/O CQ (05h) and SQ (01h) set queue 1 up; an Abort (08h) is
    /// answered with `abort_dw0` and, if asked, frees the command it names
    /// `hold` reads later (SC 07h). Every command succeeds but a Host Memory
    /// Buffer SET FEATURES (09h, FID 0Dh) the test refuses or ignores.
    fn answer(&mut self, cmd: Submission) {
        let (mut status, mut dw0) = (0, 0);
        let hmb = cmd.cdw0 & 0xff == 0x09 && cmd.cdw10 & 0xff == 0x0d;
        match cmd.cdw0 & 0xff {
            0x05 => {
                let entries = (cmd.cdw10 >> 16) as u16 + 1;
                self.io = Ring::new(0, cmd.prp1, entries, cmd.cdw11 & 0b10 != 0);
            }
            0x01 => self.io.sq = cmd.prp1,
            0x08 => {
                dw0 = self.abort_dw0;
                if self.abort_frees {
                    let cid = (cmd.cdw10 >> 16) as u16;
                    self.fail.push((cid, 0x07 << 1));
                    self.pending.push((cid, self.ticks + self.hold));
                }
            }
            _ if hmb && self.ignore_hmb => return,
            _ if hmb && self.refuse_hmb => status = INVALID_FIELD,
            _ => {}
        }
        self.admin.complete(0, (cmd.cdw0 >> 16) as u16, status, dw0);
    }
}
