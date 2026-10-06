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

use crate::window::Window;

use super::types::PER_OWNER;
use super::WindowTable;

impl WindowTable {
    // Window open answers every refusal (duplicate, past the owner's share,
    // full) with E_NOMEM, so () says all.
    #[allow(clippy::result_unit_err)]
    pub fn insert(&mut self, window: Window) -> Result<(), ()> {
        if self.find(window.owner_pid, window.window_id).is_some() {
            return Err(());
        }
        if self.held_by(window.owner_pid) >= PER_OWNER {
            return Err(());
        }
        for slot in self.entries.iter_mut() {
            if !slot.in_use {
                *slot = window;
                return Ok(());
            }
        }
        Err(())
    }
}
