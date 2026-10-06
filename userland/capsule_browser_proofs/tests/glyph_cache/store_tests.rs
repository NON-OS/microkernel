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

//! The glyph cache store: its byte budget, eviction order and bookkeeping.

use super::raster::Raster;
use super::store::{cost, Key, Store, GLYPH_CACHE_BUDGET, MAX_ENTRY};

fn key(glyph: u16, px: u32) -> Key {
    Key { face: 0x1000, len: 64, glyph, px, phase: 0 }
}

fn raster(bytes: usize) -> Raster {
    Raster { min_x: 0, min_y: 0, w: bytes as u32, h: 1, cov: vec![1; bytes] }
}

#[test]
fn the_budget_holds_through_any_run_of_inserts() {
    let (mut s, mut seed, mut peak) = (Store::new(), 0x2545_f491u32, 0);
    for i in 0..20_000 {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        let k = key((seed % 4096) as u16, seed >> 20);
        if s.get(&k).is_none() {
            s.put(k, (seed % 7 != 0).then(|| raster((seed as usize >> 8) % 6000 + 1)));
        }
        assert!(s.bytes <= GLYPH_CACHE_BUDGET, "{} bytes after insert {i}", s.bytes);
        assert_eq!(s.bytes, s.map.values().map(cost).sum::<usize>());
        peak = peak.max(s.bytes);
    }
    assert!(peak > GLYPH_CACHE_BUDGET / 2, "the run never filled the cache ({peak} bytes)");
}

#[test]
fn the_least_recently_used_go_first() {
    let (mut s, hot) = (Store::new(), key(1, 1));
    s.get(&hot);
    s.put(hot, Some(raster(4000)));
    for g in 2..2000 {
        assert!(s.get(&hot).is_some(), "the hot glyph was evicted before glyph {g}");
        s.get(&key(g, 1));
        s.put(key(g, 1), Some(raster(4000)));
    }
    assert!(s.get(&key(2, 1)).is_none(), "the oldest cold glyph survived");
}

#[test]
fn blanks_are_kept_oversized_rasters_are_not_and_clear_forgets_all() {
    let mut s = Store::new();
    s.get(&key(7, 7));
    s.put(key(7, 7), Some(raster(MAX_ENTRY)));
    assert!(s.get(&key(7, 7)).is_none() && s.bytes == 0, "an oversized raster was kept");
    s.put(key(3, 3), None);
    assert!(matches!(s.get(&key(3, 3)), Some(None)) && s.bytes > 0);
    s.clear();
    assert!(s.get(&key(3, 3)).is_none());
    assert_eq!((s.bytes, s.map.len()), (0, 0));
}
