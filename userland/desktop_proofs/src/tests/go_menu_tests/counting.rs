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

//! Which windows count as the app being open.

use super::table::index_of;
use crate::taskbar::{
    go_step, new_taskbar_state, track_window_closed, track_window_opened, GoStep,
};

/// A minimised window stays open to the window manager (no close event), so
/// the menu still focuses it, and the focus frame restores it.
#[test]
fn a_minimised_window_counts_as_open() {
    let settings = index_of(b"app.settings").unwrap();
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 30, 300, settings);
    assert_eq!(go_step(&t, Some(settings)), GoStep::Focus(settings));
}

#[test]
fn only_the_named_app_counts() {
    let term = index_of(b"app.terminal").unwrap();
    let browser = index_of(b"app.browser").unwrap();
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    assert_eq!(go_step(&t, Some(browser)), GoStep::Open);
}

#[test]
fn once_its_last_window_closes_the_app_opens_again() {
    let browser = index_of(b"app.browser").unwrap();
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 20, 200, browser);
    track_window_closed(&mut t, 200, 20);
    assert_eq!(go_step(&t, Some(browser)), GoStep::Open);
}

#[test]
fn a_service_not_in_the_table_opens() {
    let t = new_taskbar_state();
    assert_eq!(go_step(&t, None), GoStep::Open);
    assert_eq!(go_step(&t, Some(usize::MAX)), GoStep::Open);
}
