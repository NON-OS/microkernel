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
    // Drop every layer drawn from `handle`, a surface the kernel no longer
    // knows, and hand each back whole so the caller repaints where it was.
    // Unlike the reaper this does not wait: a surface that was mapped and is
    // now gone does not come back.
    pub fn drop_surface(&mut self, handle: u64, dropped: &mut [Layer]) -> usize {
        let mut n = 0;
        for slot in self.entries.iter_mut() {
            if !slot.in_use || slot.surface_handle != handle || n >= dropped.len() {
                continue;
            }
            dropped[n] = *slot;
            n += 1;
            *slot = Layer::default();
            self.count = self.count.saturating_sub(1);
        }
        n
    }
}
