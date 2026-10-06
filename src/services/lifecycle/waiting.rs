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

use crate::ipc::nonos_inbox;

/// This caller named as the one waiting on a reply inbox, for as long as it
/// waits, so the reply wakes it instead of the next tick.
pub(super) struct Waiting<'a>(&'a str);

impl<'a> Waiting<'a> {
    pub(super) fn on(inbox: &'a str) -> Self {
        if let Some(pid) = crate::process::current_pid() {
            nonos_inbox::wait_on(inbox, pid);
        }
        Waiting(inbox)
    }
}

impl Drop for Waiting<'_> {
    fn drop(&mut self) {
        nonos_inbox::unwait(self.0);
    }
}
