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

//! Which output a brightness key moves: the one whose slice of an extended
//! canvas holds the focused window's centre, named as that output; output
//! 0 when mirrored or outside every slice; that output's panel, else the
//! first listed.

use super::brightness_tests::{answer, read};
use crate::display::brightness_rule::{identity, output_at, target};
use crate::display::layout_rule::plan;

#[test]
fn the_slice_under_the_focus_names_its_output() {
    let p = plan((1280, 800), 4, &[(1920, 1080, 4)]).unwrap();
    let outputs = [0u16, 3, 0, 0];
    assert_eq!(output_at(Some(&p), &outputs, 100), 0);
    assert_eq!(output_at(Some(&p), &outputs, 1280), 3, "the second slice is output 3");
    assert_eq!(output_at(Some(&p), &outputs, 1280 + 1920), 0, "past every slice");
    assert_eq!(output_at(None, &outputs, 2000), 0, "mirrored");
}

#[test]
fn the_focused_outputs_panel_moves_else_the_first() {
    let a = answer(0, &[(identity(0), 60, vec![20, 60]), (identity(2), 30, vec![30, 90])]);
    let p = read(&a);
    assert_eq!(target(&p, 2).map(|p| p.identity), Some(identity(2)));
    assert_eq!(target(&p, 1).map(|p| p.identity), Some(identity(0)), "no panel there");
    assert_eq!(target(&[], 0), None);
}
