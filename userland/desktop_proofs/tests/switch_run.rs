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
//! A run of Alt+Tab presses: each press goes on to the next window in the
//! order as it stood at the run's first press, Shift goes back, and the run
//! ends a hold after its last press (state/ctx/switch walk.rs, session.rs).

use desktop_proofs::shell_switch::{SwitchOrder, Win};

const A: Win = (10, 1);
const B: Win = (11, 1);
const C: Win = (12, 1);

fn three() -> SwitchOrder {
    let mut o = SwitchOrder::default();
    for w in [A, B, C] {
        o.focused(w);
    }
    o
}

#[test]
fn switch_run_walks_the_order_it_began_with() {
    let mut o = three();
    assert_eq!(o.step(0, false), Some(B), "the window used before this one");
    o.focused(B);
    assert_eq!(o.step(100, false), Some(A), "on, not back to C");
    o.focused(A);
    assert_eq!(o.step(200, false), Some(C), "round to the start");
}

#[test]
fn switch_run_goes_back_with_shift() {
    let mut o = three();
    assert_eq!(o.step(0, true), Some(A));
    assert_eq!(o.step(10, true), Some(B));
    assert_eq!(o.step(20, false), Some(A));
}

#[test]
fn switch_run_needs_two_windows() {
    let mut o = SwitchOrder::default();
    assert_eq!(o.step(0, false), None);
    o.focused(A);
    assert_eq!(o.step(1, false), None, "nothing to switch to");
    assert_eq!(o.step(2, true), None);
    o.focused(B);
    assert_eq!(o.step(3, false), Some(A), "a window come since is in the next press");
}
