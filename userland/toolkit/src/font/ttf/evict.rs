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

use alloc::vec::Vec;

use super::raster::Raster;
use super::store::{cost, Key, Slot, Store, GLYPH_CACHE_BUDGET, MAX_ENTRY};

impl Store {
    /* The entry for `key`, stamped as just used; None when not cached. */
    pub fn get(&mut self, key: &Key) -> Option<&Option<Raster>> {
        self.clock += 1;
        let slot = self.map.get_mut(key)?;
        slot.used = self.clock;
        Some(&slot.raster)
    }

    /* Keep `raster` under `key`, making room first. An entry over MAX_ENTRY
    is not kept at all; the caller has already drawn it. */
    pub fn put(&mut self, key: Key, raster: Option<Raster>) {
        let slot = Slot { raster, used: self.clock };
        let need = cost(&slot);
        if need > MAX_ENTRY {
            return;
        }
        while self.bytes + need > GLYPH_CACHE_BUDGET && !self.map.is_empty() {
            self.evict_older_half();
        }
        if let Some(old) = self.map.insert(key, slot) {
            self.bytes -= cost(&old);
        }
        self.bytes += need;
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.bytes = 0;
    }

    /* Drop every entry used no later than the median stamp: at least one
    entry and about half of them, so filling the cache again takes as many
    new glyphs as it holds and the scan here stays rare. The stamps list
    (8 bytes an entry) lives only for this call. */
    fn evict_older_half(&mut self) {
        let mut stamps: Vec<u64> = self.map.values().map(|s| s.used).collect();
        let mid = stamps.len() / 2;
        let cut = *stamps.select_nth_unstable(mid).1;
        drop(stamps);
        self.map.retain(|_, s| s.used > cut);
        self.bytes = self.map.values().map(cost).sum();
    }
}
