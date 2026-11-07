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

//! The action a drop is to be: what the source offers, the destination
//! accepts, and the destination prefers, as wl_data_device 3 negotiates
//! it. Pure, so the host proofs hold it.

pub const NONE: u32 = 0;
pub const COPY: u32 = 1;
pub const MOVE: u32 = 2;
pub const ASK: u32 = 4;
/// Every action there is.
pub const ALL: u32 = COPY | MOVE | ASK;

/// The destination's preferred action when the source offers it too, else
/// the first both share, copy before move before ask; none when they share
/// none.
pub fn chosen(source: u32, dest: u32, preferred: u32) -> u32 {
    let both = source & dest & ALL;
    if preferred & both != 0 && preferred.count_ones() == 1 {
        return preferred;
    }
    [COPY, MOVE, ASK].into_iter().find(|a| both & a != 0).unwrap_or(NONE)
}

/// Whether `preferred` is one action, or none, as set_actions must name.
pub fn one_or_none(preferred: u32) -> bool {
    preferred & !ALL == 0 && preferred.count_ones() <= 1
}
