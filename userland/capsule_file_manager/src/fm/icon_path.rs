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

use nonos_toolkit::icons::IconId;

/// A point in the unit box: both axes run 0..=1000, so a glyph is resolution
/// independent and the stroker does the only division.
pub type Pt = (i16, i16);

/// A mark the manager draws: one from the toolkit's icon set, or the cube the
/// set has no counterpart for, stroked here from `CUBE`.
#[derive(Clone, Copy, PartialEq)]
pub enum Mark {
    Kit(IconId),
    Cube,
}

/// The cube as open polylines stroked end to end; a closed face repeats its
/// first point last.
pub const CUBE: &[&[Pt]] = &[
    &[(500, 100), (880, 310), (500, 520), (120, 310), (500, 100)],
    &[(120, 310), (120, 700), (500, 910), (880, 700), (880, 310)],
    &[(500, 520), (500, 910)],
];
