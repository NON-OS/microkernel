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

use super::types::{Store, StoreError};

impl Store {
    /// A handle given up. One that could write gives its file's room past
    /// its length back to the budget (budget.rs trim).
    pub fn close(&mut self, fd: u32, owner_pid: u32) -> Result<(), StoreError> {
        let slot = self.slot_mut(fd, owner_pid)?;
        let was = slot.take();
        if let Some(file) = was.filter(|e| e.writable).and_then(|e| self.files.get_mut(e.file_idx)) {
            super::budget::trim(&mut file.data);
        }
        Ok(())
    }
}
