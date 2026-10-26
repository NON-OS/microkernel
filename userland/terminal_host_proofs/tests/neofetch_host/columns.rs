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
use crate::neofetch::emblem::emblem;
use crate::neofetch::emblem_shape::EMBLEM_W;
use crate::support::{info, text};

#[test]
fn the_info_column_starts_at_the_same_offset_on_every_row() {
    let rows = two_column(&emblem(0), &info(&["alpha", "beta", "gamma"]), 2);
    for ((plain, _), want) in rows.iter().zip(["alpha", "beta", "gamma"]) {
        let line = text(plain);
        assert_eq!(line.chars().count() - want.len(), EMBLEM_W + 2, "{line:?}");
        assert!(line.ends_with(want), "{line:?}");
    }
}

/// More info rows than emblem rows: the column keeps its indent with blank
/// art on the left.
#[test]
fn a_longer_right_column_keeps_its_indent_past_the_emblem() {
    let art = emblem(0);
    let words: Vec<String> = (0..art.len() + 3).map(|i| format!("{}", i % 10)).collect();
    let right = info(&words.iter().map(String::as_str).collect::<Vec<_>>());
    let rows = two_column(&art, &right, 2);
    assert_eq!(rows.len(), right.len());
    for (plain, styled) in &rows[art.len()..] {
        assert_eq!(text(plain).chars().count(), EMBLEM_W + 2 + 1);
        assert_eq!(text(styled).trim_start().len(), 1);
    }
}

#[test]
fn a_longer_left_column_emits_the_art_alone() {
    let art = emblem(0);
    let rows = two_column(&art, &info(&["only"]), 2);
    assert_eq!(rows.len(), art.len());
    let first = text(&art[0].0);
    let pad = " ".repeat(2 + EMBLEM_W - first.chars().count());
    assert_eq!(text(&rows[0].0), format!("{first}{pad}only"));
    assert_eq!(text(&rows[1].0), text(&art[1].0).trim_end());
}
