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

//! The process monitor at every width. A narrower window kept the sidebar and
//! the inspector at full width and crushed the screen between them until its
//! cards and table drew over each other. The screen now keeps MAIN_MIN: the
//! sidebar folds to icons first, then the inspector steps aside.

use crate::pm::state::Screen;
use crate::pm_fit::{inspector, rail, sidebar_w, MAIN_MIN, RAIL_W};
use crate::metrics::{INSPECTOR_W, PANE_PAD_X, SIDEBAR_W, WIN_W};

/* What the active screen is left with at a window width. */
fn main_w(screen: Screen, w: u32) -> u32 {
    let right = if inspector(screen, w) { w - INSPECTOR_W } else { w };
    right.saturating_sub(sidebar_w(w) + PANE_PAD_X * 2)
}

#[test]
fn the_window_opens_at_the_width_these_proofs_lay_out() {
    // The manifest once carried its own copy of the size; it now takes the
    // layout's, so the full window proved here is the one the capsule opens.
    let manifest = include_str!("../../capsule_process_manager/src/pm/manifest.rs");
    assert!(manifest.contains("width: WIN_W,"));
    assert!(manifest.contains("height: WIN_H,"));
}

#[test]
fn the_full_window_is_laid_out_as_before() {
    assert!(!rail(WIN_W));
    assert_eq!(sidebar_w(WIN_W), SIDEBAR_W);
    assert!(inspector(Screen::Overview, WIN_W));
}

#[test]
fn a_narrower_window_folds_the_sidebar_first() {
    let w = SIDEBAR_W + INSPECTOR_W + MAIN_MIN - 1;
    assert_eq!(sidebar_w(w), RAIL_W);
    assert!(inspector(Screen::Overview, w));
}

#[test]
fn a_narrow_window_lets_the_inspector_go() {
    let w = RAIL_W + INSPECTOR_W + MAIN_MIN - 1;
    assert!(!inspector(Screen::Overview, w));
    assert!(!inspector(Screen::Processes, w));
}

#[test]
fn the_screen_never_gets_less_than_its_minimum_from_640_up() {
    for w in 640..=2400 {
        for screen in [Screen::Overview, Screen::Processes] {
            assert!(main_w(screen, w) + PANE_PAD_X * 2 >= MAIN_MIN, "{w}");
        }
    }
}

#[test]
fn a_screen_without_an_inspector_never_docks_one() {
    assert!(!Screen::Authority.has_inspector());
    assert!(!inspector(Screen::Authority, 4000));
}
