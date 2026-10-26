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
use super::layer::Layer;
use super::table::SceneTable;

impl SceneTable {
    /// The pointer's shape over `owner_pid`'s own surface `handle`: the
    /// layer as it is now, or None when the caller has no such surface. No
    /// process sets the shape over a surface it does not own.
    pub fn set_cursor(&mut self, owner_pid: u32, handle: u64, code: Option<u8>) -> Option<Layer> {
        let l = self
            .entries
            .iter_mut()
            .find(|l| l.in_use && l.owner_pid == owner_pid && l.surface_handle == handle)?;
        l.cursor = code;
        Some(*l)
    }
}
