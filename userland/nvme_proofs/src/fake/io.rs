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

//! How the fake controller runs the I/O commands it fetched.

use super::controller::Fake;
use crate::admin::Submission;

impl Fake {
    /// Hold an I/O command until it is due, or for good if it is stalled.
    /// An id fetched twice fails the test: the controller was let fetch a
    /// stale submission again, and would run it again.
    pub(super) fn take(&mut self, cmd: Submission) {
        let cid = (cmd.cdw0 >> 16) as u16;
        assert!(!self.fetched.contains(&cid), "I/O command {cid} fetched twice");
        self.fetched.push(cid);
        if !self.stall.contains(&cid) {
            self.pending.push((cid, self.ticks + self.hold));
        }
    }

    /// Complete stalled command `cid` now, with success: done after its wait
    /// ran out, before any Abort reaches it.
    pub fn finish(&mut self, cid: u16) {
        self.io.complete(1, cid, 0, 0);
    }

    /// Complete every held I/O command now due, with the status it is to
    /// fail with, or success.
    pub(super) fn complete_due(&mut self) {
        let ticks = self.ticks;
        let (due, held) = self.pending.iter().partition(|&&(_, at)| at <= ticks);
        self.pending = held;
        for (cid, _) in due {
            let status = self.fail.iter().find(|f| f.0 == cid).map_or(0, |f| f.1);
            self.io.complete(1, cid, status, 0);
        }
    }
}
