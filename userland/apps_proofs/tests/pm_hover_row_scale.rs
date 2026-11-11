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

//! A move inside one table row, filter chip, the search field or End Process
//! asks for no repaint and a move off it asks for one, and the scroll shifts
//! the pointer as it shifts a click, at 1x, 1.25x, 1.5x and 2x in a window
//! grown by the same scale. Its own binary, so the scale races no other test.

use apps_proofs::metrics::metrics;
use apps_proofs::pm::state::{Screen, FILTERS};
use apps_proofs::pm::ui::table_geom::{row_h, row_y, visible_rows};
use apps_proofs::pm::ui::{chips, hit::Target, hover::hover_to, insp_geom, search};
use apps_proofs::pm_window::{at, size, table, window};
use nonos_app_skeleton::EventOutcome::{Idle, Repaint};
use nonos_toolkit::design::scale::set_scale;

fn a_move_inside_a_table_row_repaints_nothing_and_one_to_the_next_repaints(q: u32) {
    let cases = [(Screen::Processes, 0), (Screen::Processes, 3), (Screen::Overview, 5)];
    for (screen, scroll) in cases {
        let (state, (t, _, _)) = (window(screen, scroll), table(screen));
        for slot in 0..visible_rows(t.h) - 1 {
            let y = t.y + row_y(slot);
            let mut hover = at(&state, t.x, y);
            assert!(hover == Some(Target::Row(scroll + slot)), "q={q}");
            assert!(hover_to(&mut hover, at(&state, t.x + t.w - 1, y + row_h() - 1)) == Idle);
            assert!(hover_to(&mut hover, at(&state, t.x, y + row_h())) == Repaint, "q={q}");
            assert!(hover == Some(Target::Row(scroll + slot + 1)), "q={q}");
        }
    }
}

fn a_move_inside_a_chip_or_a_box_repaints_nothing_and_one_off_it_repaints(q: u32) {
    let ((w, h), state) = (size(), window(Screen::Processes, 0));
    let (x0, chip_h) = (chips::origin(w, Screen::Processes).unwrap(), metrics().chip_h);
    let y = (0..h).find(|&y| at(&state, x0, y).is_some()).unwrap();
    let chip = |t| {
        let xs: Vec<u32> = (0..w).filter(|&x| at(&state, x, y) == Some(t)).collect();
        assert!(xs.len() as u32 == xs[xs.len() - 1] - xs[0] + 1, "q={q}");
        ((xs[0], y, xs.len() as u32, chip_h), t)
    };
    let mut boxes: Vec<_> = FILTERS.iter().map(|f| chip(Target::Filter(*f))).collect();
    boxes.push((search::rect(w, Screen::Processes), Target::Search));
    boxes.push((insp_geom::btn(w, h), Target::EndProcess));
    for ((x, y, bw, bh), t) in boxes {
        let mut hover = at(&state, x, y);
        assert!(hover == Some(t), "q={q}");
        assert!(hover_to(&mut hover, at(&state, x + bw - 1, y + bh - 1)) == Idle, "q={q}");
        let off = hover_to(&mut hover, at(&state, x + bw, y));
        assert!(off == Repaint && hover != Some(t), "q={q}");
    }
}

/// Both move proofs at each scale in turn, in one test so no two set the scale at once.
#[test]
fn moves_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        a_move_inside_a_table_row_repaints_nothing_and_one_to_the_next_repaints(q);
        a_move_inside_a_chip_or_a_box_repaints_nothing_and_one_off_it_repaints(q);
    }
    set_scale(4);
}
