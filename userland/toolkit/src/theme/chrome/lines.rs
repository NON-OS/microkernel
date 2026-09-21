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

//! The lines between things.

use super::super::derive::{mix, opaque};
use super::super::store::snapshot;

/// Separators, card edges, and the line under a row.
pub fn hairline() -> u32 {
    opaque(snapshot().border_argb)
}

/// A lighter line, for a division inside one surface rather than between two.
/// Two thirds of the way from the ground to the border, which reads as a hint of a
pub fn hairline_soft() -> u32 {
    let t = snapshot();
    opaque(mix(t.background_argb, t.border_argb, 168))
}
