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

//! The wallpapers kept: chosen at setup, changed in Settings, the only ones
//! a session ever reads out of the store's collection. Bit `i` keeps the
//! wallpaper at catalog index `i` (wallpaper_labels), so the set fits one
//! eight byte value for up to 64 wallpapers.

use super::wallpaper_labels::WALLPAPER_LABELS;

/// Every wallpaper in the collection, kept: what setup offers by default.
pub const ALL: u64 = if WALLPAPER_LABELS.len() >= 64 { u64::MAX } else { (1u64 << WALLPAPER_LABELS.len()) - 1 };

/// A set the store takes: at least one kept, and none past the collection.
pub fn valid(set: u64) -> bool {
    set != 0 && set & !ALL == 0
}

/// Whether the wallpaper at `index` is kept.
pub fn kept(set: u64, index: u8) -> bool {
    index < 64 && set & (1u64 << index) != 0
}

/// The kept wallpaper after `index`, going round, or `index` itself when it
/// is the only one kept.
pub fn next(set: u64, index: u8) -> u8 {
    step(set, index, 1)
}

/// The kept wallpaper before `index`, going round.
pub fn prev(set: u64, index: u8) -> u8 {
    step(set, index, WALLPAPER_LABELS.len() - 1)
}

fn step(set: u64, index: u8, by: usize) -> u8 {
    let n = WALLPAPER_LABELS.len();
    let mut i = index as usize % n;
    for _ in 0..n {
        i = (i + by) % n;
        if kept(set, i as u8) {
            return i as u8;
        }
    }
    index
}
