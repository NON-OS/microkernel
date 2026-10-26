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
//! A line too long for its width is cut with an ellipsis on a character
//! The switcher's order: a window focused goes to the front, one opened
//! behind the focused one, one closed out, and the oldest past the cap
//! (capsule_desktop_shell state/ctx/switch/order.rs).

use desktop_proofs::shell_switch::{SwitchOrder, Win};

const A: Win = (10, 1);
const B: Win = (11, 1);
const C: Win = (12, 1);

/// The order as a new run would take it.
fn listed(o: &mut SwitchOrder) -> Vec<Win> {
    o.session = None;
    o.step(0, false);
    let v = o.session.as_ref().map(|s| s.shown().0.to_vec()).unwrap_or_default();
    o.session = None;
    v
}

#[test]
fn switch_order_puts_the_focused_first() {
    let mut o = SwitchOrder::default();
    o.focused(A);
    o.focused(B);
    o.focused(C);
    assert_eq!(listed(&mut o), [C, B, A]);
    o.focused(A);
    assert_eq!(listed(&mut o), [A, C, B], "once each, moved not copied");
}

#[test]
fn switch_order_puts_an_opened_window_behind_the_focused() {
    let mut o = SwitchOrder::default();
    o.opened(A);
    o.opened(B);
    assert_eq!(listed(&mut o), [A, B]);
    o.opened(A);
    assert_eq!(listed(&mut o), [A, B], "an open already held stays where it is");
}
