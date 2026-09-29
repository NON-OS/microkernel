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

/* Digits of a stack key (see stack), one per nesting level: a class in the
 * high 12 bits and, for a positioned box or a context, its place in tree
 * order in the low 20, so two such boxes of one class never interleave.
 * Within a context, DECOR sorts before every z-index, negative ones fall
 * below FLOW, and LAYER is z-index 0. */
pub(super) const LEVELS: usize = 8;
pub(super) const DECOR: u32 = 0;
const FLOW: u32 = 2000 << 20;
pub(super) const LAYER: u32 = 2001;

/// A paint order key: fragments paint in ascending order.
pub(crate) type Key = [u32; LEVELS];

/// The key of the root context: every digit FLOW.
pub(super) const ROOT: Key = [FLOW; LEVELS];

/// The digit of class `class` for the box with tree position `seq`.
pub(super) fn digit(class: u32, seq: usize) -> u32 {
    class << 20 | seq.min(0xF_FFFF) as u32
}

/// The class of z-index `z` (-999..=999): odd, around FLOW's 2000.
pub(super) fn z_class(z: i32) -> u32 {
    (2 * (z.clamp(-999, 999) + 1000) + 1) as u32
}
