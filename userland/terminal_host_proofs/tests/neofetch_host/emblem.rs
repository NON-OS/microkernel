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

//! The emblem's rows and their declared width.

use crate::neofetch::emblem::emblem;
use crate::neofetch::emblem_shape::{EMBLEM_H, EMBLEM_W};
use crate::support::text;

/// The emblem is a grid, not a picture: one short row slides every info line
/// beside it out of the column.
#[test]
fn every_row_is_exactly_the_declared_width() {
    let rows = emblem(0x07090B);
    assert_eq!(rows.len(), EMBLEM_H / 2);
    for (index, (plain, _)) in rows.iter().enumerate() {
        assert_eq!(text(plain).chars().count(), EMBLEM_W, "row {index}");
    }
}

/// The rows are half blocks, so the byte length is far past the column
/// count. Anything measuring with `len()` would indent the info column by
/// three times the intended amount.
#[test]
fn the_declared_width_is_columns_and_not_bytes() {
    assert!(emblem(0x07090B).iter().any(|(plain, _)| plain.len() > EMBLEM_W));
}
