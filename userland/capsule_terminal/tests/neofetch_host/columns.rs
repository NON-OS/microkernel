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

//! Composing the emblem beside the info column.

use crate::neofetch::compose::two_column;
use crate::neofetch::logo::{LOGO, LOGO_W};
use crate::support::{info, text};

#[test]
fn the_info_column_starts_at_the_same_offset_on_every_row() {
    let rows = two_column(&LOGO, &info(&["alpha", "beta", "gamma"]), 2);
    for (row, want) in rows.iter().zip(["alpha", "beta", "gamma"]) {
        let line = text(row);
        let at = line.char_indices().find(|(_, c)| *c == want.as_bytes()[0] as char);
        assert_eq!(line.chars().count() - want.chars().count(), LOGO_W + 2, "{line:?}");
        assert!(at.is_some());
        assert!(line.ends_with(want), "{line:?}");
    }
}

/// More info rows than emblem rows is the normal case: the strip has to keep
/// running with blank art on the left.
#[test]
fn a_longer_right_column_keeps_its_indent_past_the_emblem() {
    let right = info(&["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"]);
    let rows = two_column(&LOGO, &right, 2);
    assert_eq!(rows.len(), right.len());
    for row in &rows {
        assert_eq!(text(row).chars().count(), LOGO_W + 2 + 1);
    }
    assert!(!text(&rows[9]).contains('│'));
}

#[test]
fn a_longer_left_column_emits_the_art_alone() {
    let rows = two_column(&LOGO, &info(&["only"]), 2);
    assert_eq!(rows.len(), LOGO.len());
    assert_eq!(text(&rows[0]), format!("{}  only", LOGO[0]));
    assert_eq!(text(&rows[1]), LOGO[1].trim_end());
}
