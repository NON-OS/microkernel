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
//! What a change of the menu bar's drop-down composes: the bar, whose open
//! title is lit, and the drop-down with its shadow where it was and where it
//! is now. Opening, closing or switching a drop-down repainted the shell
//! whole, and the whole 1920x1080 screen was composed twice for each (a
//! development guest, TCG: 352 and 679 ms). Pure; held in desktop_proofs.

use super::focus_parts::Area;

/// The bar first, then the drop-down's place before and after, when there
/// was or is one; the count of them that hold.
pub fn dropdown_parts(bar: Area, was: Option<Area>, now: Option<Area>) -> ([Area; 3], usize) {
    let mut parts = [bar; 3];
    let mut n = 1;
    for a in [was, now].into_iter().flatten() {
        parts[n] = a;
        n += 1;
    }
    (parts, n)
}
