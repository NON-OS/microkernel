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
//! The switcher's order: a window closed goes out, and the oldest goes past
//! the cap (capsule_desktop_shell state/ctx/switch/order.rs).

use desktop_proofs::shell_switch::{SwitchOrder, Win, ORDER_MAX};

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
fn switch_order_drops_a_closed_window() {
    let mut o = SwitchOrder::default();
    for w in [A, B, C] {
        o.focused(w);
    }
    o.closed(B);
    assert_eq!(listed(&mut o), [C, A]);
    let same_id_other_pid = (99, 1);
    o.closed(same_id_other_pid);
    assert_eq!(listed(&mut o), [C, A], "only the pid tells two instances apart");
}

#[test]
fn switch_order_keeps_the_newest_past_the_cap() {
    let mut o = SwitchOrder::default();
    for pid in 0..ORDER_MAX as u32 + 3 {
        o.focused((pid + 1, 1));
    }
    let l = listed(&mut o);
    assert_eq!(l.len(), ORDER_MAX);
    assert_eq!(l[0], (ORDER_MAX as u32 + 3, 1));
    assert!(!l.contains(&(1, 1)), "the least recent went");
    o.opened((500, 1));
    assert_eq!(listed(&mut o).len(), ORDER_MAX, "an open keeps the cap too");
}
