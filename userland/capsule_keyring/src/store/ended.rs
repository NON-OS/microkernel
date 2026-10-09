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

/*
 * A key answers only the pid that stored it, and the kernel counts pids up
 * through the whole 32-bit range before it hands one out again, so once its
 * owner ends nobody can read, use or delete it. It stayed
 * anyway: secret bytes held for the life of the boot, and a place taken.
 * Eight owners that ended at their share filled the keyring, and no program
 * could store or open a key again until a reboot. These are dropped, each
 * wiped as its owner's own delete would have wiped it.
 */

use super::types::{Store, MAX_KEYS};

impl Store {
    /// Drop every key whose owner `alive` says has ended, and say how many
    /// were dropped.
    pub fn drop_ended(&mut self, alive: impl Fn(u32) -> bool) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, entry| {
            if alive(entry.owner_pid) {
                return true;
            }
            super::wipe::secure_wipe(&mut entry.data);
            false
        });
        before.saturating_sub(self.entries.len())
    }

    /// Whether no key can be stored for anyone.
    pub fn is_full(&self) -> bool {
        self.entries.len() >= MAX_KEYS
    }
}
