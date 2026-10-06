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

use alloc::collections::BTreeMap;

use super::raster::Raster;

/// The bytes the glyph cache may account for at once.
pub const GLYPH_CACHE_BUDGET: usize = 2 * 1024 * 1024;

/*
 * One rasterisation. `face` and `len` are the address and length of the
 * face's font data: two faces never share live data, so this tells a bold
 * cut from its regular one even where every metric matches. `phase` is the
 * quarter-pixel x offset the glyph was drawn at, or ORIGIN for the chrome
 * path, which draws every glyph at the pen origin.
 */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Key {
    pub face: usize,
    pub len: usize,
    pub glyph: u16,
    pub px: u32,
    pub phase: u8,
}

pub(super) struct Slot {
    pub raster: Option<Raster>,
    pub used: u64,
}

/*
 * What one entry is charged: its coverage plus three times the entry itself.
 * A map node holds eleven entries and is never less than five full; with the
 * edges of the nodes above it, one entry's share of the tree stays under
 * three entries. An entry with no raster is a glyph with nothing to draw (a
 * space), kept so it is not outlined again.
 */
pub(super) fn cost(slot: &Slot) -> usize {
    let entry = 3 * core::mem::size_of::<(Key, Slot)>();
    entry + slot.raster.as_ref().map_or(0, |r| r.cov.len())
}

/* The coverage-to-budget cap for one entry: a glyph bigger than an eighth of
the budget is drawn and dropped, so a few huge headings cannot flush every
body glyph out of the cache. */
pub(super) const MAX_ENTRY: usize = GLYPH_CACHE_BUDGET / 8;

/* Rasters by key, bounded by GLYPH_CACHE_BUDGET: `used` stamps each hit and
a full cache drops its least recently used half (evict.rs). */
pub(super) struct Store {
    pub map: BTreeMap<Key, Slot>,
    pub bytes: usize,
    pub clock: u64,
}

impl Store {
    pub const fn new() -> Self {
        Store { map: BTreeMap::new(), bytes: 0, clock: 0 }
    }
}
