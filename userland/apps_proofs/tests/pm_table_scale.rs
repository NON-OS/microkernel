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

//! The process tables are the shared Table, and a click names what it drew:
//! a header names its column, a row its row under the scroll, and the rule
//! under the header and the room past the last row name nothing, at 1x, 1.25x,
//! 1.5x and 2x in a window grown by the same scale. Its own binary, so setting
//! the scale races no other test.

use apps_proofs::pm::state::Screen;
use apps_proofs::pm::ui::hit::Target;
use apps_proofs::pm::ui::table_geom::{row_h, row_y, shown};
use apps_proofs::pm_window::{at, table, window};
use nonos_toolkit::design::scale::set_scale;

fn a_header_hit_names_the_column_drawn_under_it_and_the_rule_names_nothing(q: u32) {
    for screen in [Screen::Processes, Screen::Overview] {
        let (state, (t, cols, laid)) = (window(screen, 0), table(screen));
        assert!(laid[0].0 == 0 && laid.windows(2).all(|p| p[0].0 + p[0].1 == p[1].0));
        assert!(laid.last().map(|(x, w)| x + w) == Some(t.w), "q={q}");
        for dx in 0..t.w {
            let k = laid.iter().position(|(x, w)| dx >= *x && dx < x + w).unwrap();
            for y in [t.y, t.y + row_h() - 1] {
                assert!(at(&state, t.x + dx, y) == Some(Target::Sort(cols[k])), "q={q}");
            }
            assert!(at(&state, t.x + dx, t.y + row_h()).is_none(), "q={q}: the rule");
        }
    }
}

fn a_row_hit_names_its_row_under_the_scroll_and_nothing_past_the_last(q: u32) {
    let cases = [(Screen::Processes, 0), (Screen::Processes, 33), (Screen::Overview, 37)];
    for (screen, scroll) in cases {
        let (state, (t, _, _)) = (window(screen, scroll), table(screen));
        let n = shown(scroll, 40, t.h);
        for (slot, dy, dx) in (0..n).flat_map(|s| [(s, 0, 0), (s, row_h() - 1, t.w - 1)]) {
            let named = at(&state, t.x + dx, t.y + row_y(slot) + dy);
            assert!(named == Some(Target::Row(scroll + slot)), "q={q}: slot {slot}");
        }
        let below = t.y + row_y(n);
        assert!(below >= t.y + t.h || at(&state, t.x, below).is_none(), "q={q}");
    }
}

/// Both hit proofs at each scale in turn, in one test so no two set the scale at once.
#[test]
fn hits_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        a_header_hit_names_the_column_drawn_under_it_and_the_rule_names_nothing(q);
        a_row_hit_names_its_row_under_the_scroll_and_nothing_past_the_last(q);
    }
    set_scale(4);
}
