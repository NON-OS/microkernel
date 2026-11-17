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
//! sidebar folds to icons first, then the inspector steps aside. The widths
//! are proven at every display scale in `tests/pm_fit_scale.rs`.

use crate::pm::state::Screen;
use crate::pm_fit::inspector;

#[test]
fn the_window_opens_at_the_width_these_proofs_lay_out() {
    // The manifest once carried its own copy of the size; it now takes the
    // layout's, so the full window proved here is the one the capsule opens.
    let manifest = include_str!("../../capsule_process_manager/src/pm/manifest.rs");
    assert!(manifest.contains("width: WIN_W,"));
    assert!(manifest.contains("height: WIN_H,"));
}

#[test]
fn a_screen_without_an_inspector_never_docks_one() {
    assert!(!Screen::Authority.has_inspector());
    assert!(!inspector(Screen::Authority, 4000));
}
