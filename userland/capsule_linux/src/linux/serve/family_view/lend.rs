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

/* The view a family lends /proc for a call. */

use nonos_libc::ForeignFrame;

use crate::linux::file::{self, View};

use super::super::family::Family;
use super::proc::proc_of;

/* The personality itself: the namespace's pid 1, never shown under /proc. */
pub(super) const HOST_NS: u32 = 1;

impl Family {
    pub(crate) fn lend_view(&mut self, i: usize, frame: &ForeignFrame) {
        self.note_exit(i, frame);
        if !file::needs_view(&self.guests[i], frame.nr, frame.args()) {
            return;
        }
        let me = self.ns.outward(self.guests[i].pid);
        let thread = self.ns.outward(frame.pid);
        let n = self.guests.len();
        let procs = (0..n).map(|j| proc_of(&self.guests, &mut self.ns, j, j == i)).collect();
        file::lend_view(View { me, thread, procs });
    }

    pub(crate) fn take_view(&mut self) {
        file::lend_view(View::default());
    }
}
