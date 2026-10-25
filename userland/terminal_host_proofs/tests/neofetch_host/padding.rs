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

//! Trailing padding and empty columns.

use crate::neofetch::compose::two_column;
use crate::neofetch::logo::LOGO;
use crate::support::text;

/// A blank emblem row against a blank info row must not leave a line of
/// invisible padding in the scrollback.
#[test]
fn trailing_padding_never_survives() {
    let rows = two_column(&LOGO, &[], 2);
    assert_eq!(text(&rows[0]), "");
    for row in &rows {
        assert!(!text(row).ends_with(' '), "{:?}", text(row));
    }
}

#[test]
fn two_empty_columns_compose_to_nothing() {
    assert!(two_column(&[], &[], 2).is_empty());
    assert!(two_column(&[], &[], 0).is_empty());
}
