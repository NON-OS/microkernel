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

//! A thread that has gone, taking its signal state and its waits with it.

use super::handle::Guest;

impl Guest {
    /// A thread that has gone takes its signal state and its waits with it.
    pub fn forget_thread(&mut self, tid: u32) {
        let _ = self.leave_waits(tid);
        self.signals.threads.retain(|t| t.tid != tid);
        self.signals.pending.retain(|(t, _)| *t != tid);
    }
}
