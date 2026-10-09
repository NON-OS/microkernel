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

use super::{Context, SessionState};

impl Context {
    /// Lock a session whose owner `alive` says has ended, as that owner's
    /// own end would have, and give back the key it was opened with. Only
    /// the owner can end its session, so one that ended without ending it
    /// left the machine unlocked in its name, and every later start was
    /// refused as busy until a reboot.
    pub fn end_if_owner_ended(&mut self, alive: impl Fn(u32) -> bool) -> Option<u32> {
        match self.state {
            SessionState::Unlocked { owner_pid, key_id, .. } if !alive(owner_pid) => {
                self.state = SessionState::Locked;
                Some(key_id)
            }
            _ => None,
        }
    }
}
