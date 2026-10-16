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

//! The emblem rows and their declared width.

use crate::neofetch::logo::{LOGO, LOGO_W};

/// The emblem is a grid, not a picture: one short row slides every info line
/// under it out of the column.
#[test]
fn every_logo_row_is_exactly_the_declared_width() {
    for (index, art) in LOGO.iter().enumerate() {
        assert_eq!(art.chars().count(), LOGO_W, "row {index}: {art:?}");
    }
}

/// The rows are box drawing glyphs, so the byte length is far past the column
/// count. Anything measuring with `len()` would indent the info column by
/// three times the intended amount.
#[test]
fn the_declared_width_is_columns_and_not_bytes() {
    assert!(LOGO.iter().any(|art| art.len() > LOGO_W));
    assert_eq!(LOGO[0].len(), LOGO_W);
}
