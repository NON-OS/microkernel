// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A shell menu closes when focus moves to another window (one opening,
//! Alt+Tab), and not when the window manager says again which window has
//! it (capsule_desktop_shell server/wm_notify_switch.rs, from
//! state/ctx/switch/order.rs `moves`).

use desktop_proofs::shell_switch::{SwitchOrder, Win};

const A: Win = (10, 1);
const B: Win = (11, 1);
const A_AGAIN: Win = (12, 1);

#[test]
fn focus_moving_to_another_window_closes_a_menu() {
    let mut o = SwitchOrder::default();
    assert!(o.moves(A), "the first window to take focus");
    o.focused(A);
    o.opened(B);
    assert!(o.moves(B), "a window behind the focused one");
    assert!(o.moves(A_AGAIN), "a second instance of the same app");
}

#[test]
fn focus_told_again_keeps_it_open() {
    let mut o = SwitchOrder::default();
    o.focused(A);
    o.focused(B);
    assert!(!o.moves(B));
    o.focused(A);
    assert!(!o.moves(A), "the one now in front");
}
