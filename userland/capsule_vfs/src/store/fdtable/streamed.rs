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

//! Files whose bytes stay on the device (blk::streamed).

use super::types::Store;

impl Store {
    /// The size a file reports: its bytes, or the length on the device.
    pub(super) fn size_of(&self, idx: usize) -> u64 {
        let f = &self.files[idx];
        f.streamed.as_ref().map_or(f.data.len() as u64, |e| e.len)
    }

    /// Whether the file at `idx` is served from the device.
    pub(super) fn is_streamed(&self, idx: usize) -> bool {
        self.files[idx].streamed.is_some()
    }
}
