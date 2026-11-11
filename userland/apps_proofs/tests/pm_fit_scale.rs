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

//! The process monitor at every width, at 1x, 1.25x, 1.5x and 2x: the full
//! window grown by the scale is laid out as it is at 1x, a narrower one folds
//! the sidebar first, a narrow one lets the inspector go, and the screen never
//! gets less than its minimum. Its own binary, so setting the scale races no
//! other test.

use apps_proofs::metrics::metrics;
use apps_proofs::pm::state::Screen;
use apps_proofs::pm_fit::{inspector, rail, sidebar_w, MAIN_MIN, RAIL_W};
use apps_proofs::pm_window::size;
use nonos_toolkit::design::scale::{scale, set_scale};

/* What the active screen is left with at a window width. */
fn main_w(screen: Screen, w: u32) -> u32 {
    let m = metrics();
    let right = if inspector(screen, w) { w - m.inspector_w } else { w };
    right.saturating_sub(sidebar_w(w) + m.pane_pad_x * 2)
}

fn every_width_is_shared_out_as_at_1x(q: u32) {
    let (s, m, full) = (scale(), metrics(), size().0);
    let (rail_w, main_min) = (s.px(RAIL_W), s.px(MAIN_MIN));
    assert!(!rail(full) && sidebar_w(full) == m.sidebar_w, "q={q}: the full window");
    assert!(inspector(Screen::Overview, full), "q={q}: the full window docks");
    let narrower = m.sidebar_w + m.inspector_w + main_min - 1;
    assert_eq!(sidebar_w(narrower), rail_w, "q={q}: the sidebar folds first");
    assert!(inspector(Screen::Overview, narrower), "q={q}: the inspector stays");
    let narrow = rail_w + m.inspector_w + main_min - 1;
    assert!(!inspector(Screen::Overview, narrow), "q={q}: the inspector goes");
    assert!(!inspector(Screen::Processes, narrow), "q={q}: the inspector goes");
    assert!(!inspector(Screen::Authority, s.px(4000)), "q={q}: Authority never docks");
}

fn the_screen_never_gets_less_than_its_minimum_from_640_up(q: u32) {
    let (s, pad) = (scale(), metrics().pane_pad_x);
    for w in s.px(640)..=s.px(2400) {
        for screen in [Screen::Overview, Screen::Processes] {
            assert!(main_w(screen, w) + pad * 2 >= s.px(MAIN_MIN), "q={q}: {w}");
        }
    }
}

/// Both width proofs at each scale in turn, in one test so no two set the scale at once.
#[test]
fn widths_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        every_width_is_shared_out_as_at_1x(q);
        the_screen_never_gets_less_than_its_minimum_from_640_up(q);
    }
    set_scale(4);
}
