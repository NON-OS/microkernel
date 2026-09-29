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

use crate::browser::css::Computed;

/// The four corner radii of a box `w` px wide, in px, top-left first
/// and clockwise. A percentage resolves against the box width; corners are
/// drawn circular, and the painter shrinks any radius past half the
/// shorter side, so 50% rounds a square into a circle and a wide box into
/// a pill. Negative values are invalid and draw square.
pub(crate) fn radii(s: &Computed, w: i32) -> [u16; 4] {
    s.radius.map(|r| r.resolve(w).unwrap_or(0).clamp(0, u16::MAX as i32) as u16)
}
