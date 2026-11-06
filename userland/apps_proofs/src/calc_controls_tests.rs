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

//! The calculator's controls: in every mode a click or a hover names the control drawn there,
//! every key and kept calculation is in the table, and no two overlap.

use crate::calc::buttons::grid;
use crate::calc::convert::CATEGORIES;
use crate::calc::history::CAP;
use crate::calc::hit::Hit;
use crate::calc::mode::{Mode, MODES};
use crate::calc::state::State;
use crate::calc::ui::history_geom::capacity;
use crate::calc::ui::target::{at, targets};
use crate::calc_hover_tests::mid;

pub fn window(mode: Mode, category: usize, calculations: usize) -> State {
    let mut s = State::new();
    s.set_mode(mode);
    s.set_category(category);
    (0..calculations).for_each(|_| s.history.push(b"1 + 1", 2));
    s
}

/// Each mode, the converter on each category, and History with one and with all.
pub fn windows() -> Vec<State> {
    let modes = MODES.into_iter().map(|m| window(m, 0, 0));
    let cats = (1..CATEGORIES.len()).map(|c| window(Mode::Convert, c, 0));
    modes.chain(cats).chain([1, CAP].map(|n| window(Mode::History, 0, n))).collect()
}

#[test]
fn every_control_is_named_from_corner_to_corner_and_not_past_its_edges() {
    for s in windows() {
        for (t, r) in targets(&s) {
            let (x0, y0, x1, y1) = (r.x as i32, r.y as i32, (r.x + r.w) as i32, (r.y + r.h) as i32);
            let named = |(x, y)| at(&s, x, y) == Some(t);
            let inside = [mid(r), (x0, y0), (x1 - 1, y1 - 1)];
            assert!(r.w > 0 && inside.into_iter().all(named), "{r:?} in {}", s.mode.label());
            assert!(!named((x1, y0)) && !named((x0, y1)), "{r:?} in {}", s.mode.label());
        }
    }
}

#[test]
fn no_two_controls_overlap_and_every_key_and_calculation_is_in_the_table() {
    for s in windows() {
        let ts = targets(&s);
        for (i, (_, a)) in ts.iter().enumerate() {
            for (_, b) in &ts[i + 1..] {
                let beside = a.x + a.w <= b.x || b.x + b.w <= a.x;
                let apart = beside || a.y + a.h <= b.y || b.y + b.h <= a.y;
                assert!(apart, "{a:?} and {b:?} in {}", s.mode.label());
            }
        }
        let keys = ts.iter().filter(|(t, _)| matches!(t, Hit::Key(..))).count();
        assert_eq!(keys, grid(s.mode).iter().map(|r| r.len()).sum::<usize>());
        let page = if s.mode == Mode::History { capacity(s.view.1) } else { 0 };
        let rows = ts.iter().filter(|(t, _)| matches!(t, Hit::Row(_))).count();
        assert_eq!(rows, s.history.len().min(page));
    }
}
