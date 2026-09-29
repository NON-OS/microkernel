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

use super::store::Store;

/* Times one image may come back after being evicted while its own box was
 * on or near the screen. Past it the images shown together outgrow the
 * budget, and fetching one more would only evict another that is also
 * shown, so it keeps its fallback. An image evicted after it scrolled out
 * of that window comes back as often as it is scrolled to. */
pub(super) const MAX_THRASH: u8 = 3;

/// Per-image record of how it left and came back, in Entry::revival.
#[derive(Default)]
pub(super) struct Revival {
    /* Its box was near the screen at the last requeue pass. */
    pub near: bool,
    /* It was evicted while near: the screen itself outgrew the budget. */
    pub thrash: bool,
    pub refetched: u8,
}

impl Store {
    /// Record an eviction: one that takes an image still near the screen
    /// counts toward MAX_THRASH when the image comes back.
    pub(super) fn note_evicted(&mut self, url: &str) {
        if let Some(e) = self.entries.get_mut(url) {
            e.revival.thrash = e.revival.near;
        }
    }
}
