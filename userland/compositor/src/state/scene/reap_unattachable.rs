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
    // Drop every layer whose surface has not attached for `threshold` paints in
    // a row (its owner died, or released the surface without a scene remove),
    // and hand each dropped layer back whole: the caller forgets its handle and
    // repaints its rectangle. The rectangle matters as much as the handle. The
    // layer's last pixels are still on screen, and only a repaint of where it
    // was takes them off; reporting the handle alone left a dead window drawn
    // until the next periodic full frame, several seconds later.
    pub fn reap_unattachable(
        &mut self,
        attached: &[u64],
        threshold: u16,
        dropped: &mut [Layer],
    ) -> usize {
        let mut n = 0;
        for slot in self.entries.iter_mut() {
            if !slot.in_use {
                continue;
            }
            if attached.contains(&slot.surface_handle) {
                slot.miss_count = 0;
                continue;
            }
            slot.miss_count = slot.miss_count.saturating_add(1);
            if slot.miss_count >= threshold && n < dropped.len() {
                dropped[n] = *slot;
                n += 1;
                *slot = Layer::default();
                self.count = self.count.saturating_sub(1);
            }
        }
        n
    }
}
