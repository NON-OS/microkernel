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

//! A Linux package's pack, held by the runs reading it. capsule_linux keeps
//! a handle on each pack a run reads open for the run (store/pack/table.rs
//! there): while one is, no other process may replace or remove that pack,
//! so an update or an uninstall is told it is in use, never pulled out from
//! under a running program. The run's handles close when it ends, a crashed
//! run's when it is reaped (reap.rs), and the hold goes with them.

use super::types::Store;

/// Where the packs are: the hold is theirs alone.
pub const HELD_PREFIX: &str = "/linux-packs/";

impl Store {
    /// Whether a process other than `owner` holds `path` open, for a path
    /// the hold covers.
    pub fn held_by_another(&self, path: &str, owner: u32) -> bool {
        if !path.starts_with(HELD_PREFIX) {
            return false;
        }
        let Some(idx) = self.find(path) else { return false };
        self.fds.iter().flatten().any(|fd| fd.file_idx == idx && fd.owner_pid != owner)
    }
}
