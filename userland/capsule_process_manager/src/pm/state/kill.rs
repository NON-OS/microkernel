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

use nonos_libc::mk_kill;

use super::notes::{kill_note, ARMED, NO_SELECTION, PROTECTED};
use super::{State, SIGTERM};

impl State {
    // Cancel any armed kill. Called whenever the user navigates or re-sorts, so
    // a pending confirmation never carries over to a different selection. The
    // status strip shows `notice`, so the prompt or outcome it holds goes too:
    // it was about the selection the user just left.
    pub(super) fn disarm(&mut self) {
        self.pending_pid = 0;
        self.notice = b"";
    }

    // End the selected process, in two steps. The first press arms the exact
    // pid; the same press again confirms and sends it. Core system processes
    // and this window are refused outright: ending the compositor, window
    // manager, input router or a core service would strand the whole session,
    // so the monitor never lets a keypress do it. Ordinary apps stay endable.
    // The kernel still enforces the ProcessControl authority on top of this,
    // and what it answered is what the strip says.
    pub fn end_selected(&mut self) {
        let Some(pid) = (self.selected_pid != 0).then_some(self.selected_pid) else {
            self.notice = NO_SELECTION;
            return;
        };
        if self.rows.iter().find(|r| r.pid == pid).is_some_and(|r| self.is_protected(r)) {
            self.disarm();
            self.notice = PROTECTED;
            return;
        }
        if self.pending_pid == pid {
            let rc = mk_kill(pid as u64, SIGTERM);
            self.disarm();
            self.notice = kill_note(rc);
            self.refresh();
            return;
        }
        self.pending_pid = pid;
        self.notice = ARMED;
    }
}
