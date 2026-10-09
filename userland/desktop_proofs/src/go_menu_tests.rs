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

//! A menubar item naming an app (Go > Terminal, Browser, Settings, and the
//! others that open an app) brings that app's window forward when it has one
//! and opens a window only when it has none.

use crate::shell_apps::LAUNCHER_APPS;
use crate::taskbar::{
    go_step, new_taskbar_state, raise_tracked, track_window_closed, track_window_opened, GoStep, Reach,
};

fn index_of(service: &[u8]) -> Option<usize> {
    LAUNCHER_APPS.iter().position(|a| a.service == service)
}

#[test]
fn every_go_item_names_an_app_in_the_launcher_table() {
    for service in [&b"app.terminal"[..], b"app.browser", b"app.settings"] {
        assert!(index_of(service).is_some(), "{:?}", core::str::from_utf8(service));
    }
}

#[test]
fn an_app_with_no_window_opens_one() {
    let t = new_taskbar_state();
    assert_eq!(go_step(&t, index_of(b"app.terminal")), GoStep::Open);
}

#[test]
fn an_app_with_a_window_is_brought_forward_not_opened_again() {
    let term = index_of(b"app.terminal").unwrap();
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    // A click after a click: still the one window, never a second.
    assert_eq!(go_step(&t, Some(term)), GoStep::Focus(term));
    assert_eq!(go_step(&t, Some(term)), GoStep::Focus(term));
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Taken), Some(100));
}

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
