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
    /// Drop `owner_pid`'s layer in band `z` alone, and hand it back.
    pub fn drop_band(&mut self, owner_pid: u32, z: u32) -> Option<Layer> {
        let slot =
            self.entries.iter_mut().find(|l| l.in_use && l.owner_pid == owner_pid && l.z == z)?;
        let gone = *slot;
        *slot = Layer::default();
        self.count = self.count.saturating_sub(1);
        Some(gone)
    }
}
