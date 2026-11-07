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
    /// Drop `owner_pid`'s keyed layer showing `handle` in band `z`, and hand
    /// it back. Every other layer the project holds stays as it is.
    pub fn drop_layer(&mut self, owner_pid: u32, z: u32, handle: u64) -> Option<Layer> {
        let slot = self.entries.iter_mut().find(|l| {
            l.in_use
                && l.keyed
                && l.owner_pid == owner_pid
                && l.z == z
                && l.surface_handle == handle
        })?;
        let gone = *slot;
        *slot = Layer::default();
        self.count = self.count.saturating_sub(1);
        Some(gone)
    }
}
