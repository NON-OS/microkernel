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

use capsule_browser_proofs::browser::ui::sidebar_geom::{rows, RowKind};
use nonos_toolkit::widgets::{Controls, Step};

use super::kit::{at_scale, changed, look, mid, paint, side};
use super::press_tests::scales;

#[test]
fn only_the_row_on_show_is_marked() {
    for (q, rail) in scales() {
        let _g = at_scale(q);
        let (s, mut c) = (side(rail, 1400), Controls::acting_on_press());
        let one = paint(rail, s, &look(Some(RowKind::Tabs), false), &mut c);
        let two = paint(rail, s, &look(Some(RowKind::History), false), &mut c);
        let ours = |k| k == Some(RowKind::Tabs) || k == Some(RowKind::History);
        let diff = changed(&one, &two, s);
        assert!(!diff.is_empty() && diff.iter().all(|&(x, y)| ours(c.at(x, y))), "{rail} {q}");
    }
}

#[test]
fn a_full_window_s_new_group_takes_nothing() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let (s, k, mut c) = (side(false, 1400), look(None, true), Controls::acting_on_press());
        let unlit = paint(false, s, &k, &mut c);
        let row = rows(false, s, 2).into_iter().find(|r| r.kind == RowKind::NewGroup).unwrap();
        let (x, y) = mid(row.rect);
        c.moved(x, y);
        assert_eq!(paint(false, s, &k, &mut c), unlit, "lit at {q}");
        assert!(!matches!(c.pressed(x, y), Step::Act(_)) && c.at(x, y).is_none(), "at {q}");
    }
}
