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

//! Which chunk to ask for next: the lowest one not held and not in flight,
//! while fewer than `width` are in flight. Pure, so a fetch over one stream
//! (width 1) and one over several take chunks in the same order.

/// The next chunk to ask for, or None when `width` are in flight or every
/// chunk is held or asked for.
pub fn next(held: &[bool], flying: &[usize], width: usize) -> Option<usize> {
    if flying.len() >= width {
        return None;
    }
    (0..held.len()).find(|i| !held[*i] && !flying.contains(i))
}
