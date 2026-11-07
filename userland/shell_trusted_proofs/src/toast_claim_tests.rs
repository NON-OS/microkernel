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

//! The toast panel's window over the panel, so a press on it reaches the
//! shell: counted closed or open only as the window manager answered, so a
//! call that failed is made again on the next sync.

use crate::toast_claim::{after_close, after_open, CLOSE_REFUSED};

const PANEL: (u32, u32, u32, u32) = (1500, 700, 400, 120);

#[test]
fn a_close_left_unanswered_keeps_the_window_held_and_is_made_again() {
    assert_eq!(after_close(Some(PANEL), Err("wm close failed")), Some(PANEL));
    assert_eq!(after_close(Some(PANEL), Err("wm short close response")), Some(PANEL));
}

#[test]
fn a_close_answered_or_refused_lets_the_window_go() {
    assert_eq!(after_close(Some(PANEL), Ok(())), None);
    assert_eq!(after_close(Some(PANEL), Err(CLOSE_REFUSED)), None, "no such window: gone");
}

#[test]
fn an_open_counts_only_once_it_opened() {
    assert_eq!(after_open(PANEL, true), Some(PANEL));
    assert_eq!(after_open(PANEL, false), None, "tried again on the next sync");
}

#[test]
fn the_refusal_is_the_one_the_window_manager_client_gives() {
    const CLOSE: &str = include_str!("../../capsule_desktop_shell/src/wm_client/window_close.rs");
    assert!(CLOSE.contains("return Err(\"wm rejected window_close\");"));
    assert_eq!(CLOSE_REFUSED, "wm rejected window_close");
    const PRESS: &str = include_str!("../../capsule_desktop_shell/src/render/toasts/press.rs");
    assert!(PRESS.contains("after_close(ctx.toast_view.toast_window, closed)"));
    assert!(PRESS.contains("after_open(p, opened.is_ok())"));
    assert!(!PRESS.contains("let _ = wm_client::window_"), "no answer dropped");
}
