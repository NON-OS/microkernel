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
//! A run of Alt+Tab presses ends a hold after its last press, and a press
//! after that starts a new run on the order as it is then (state/ctx/switch
//! walk.rs, session.rs).

use desktop_proofs::shell_switch::{SwitchOrder, Win};

const A: Win = (10, 1);
const B: Win = (11, 1);
const C: Win = (12, 1);
const HOLD: i64 = 1200;

fn three() -> SwitchOrder {
    let mut o = SwitchOrder::default();
    for w in [A, B, C] {
        o.focused(w);
    }
    o
}

#[test]
fn switch_run_ends_a_hold_after_its_last_press() {
    let mut o = three();
    o.step(0, false);
    o.step(1000, false);
    let run = o.session.as_ref().expect("a run");
    assert_eq!(run.until_ms(), 1000 + HOLD, "each press holds it again");
    assert!(run.live(1000 + HOLD - 1) && !run.live(1000 + HOLD));
    assert!(!o.expire(1000 + HOLD - 1));
    assert!(o.expire(1000 + HOLD), "the strip goes");
    assert!(o.session.is_none() && !o.expire(5000), "once");
}

#[test]
fn switch_run_after_the_hold_starts_from_the_new_order() {
    let mut o = three();
    assert_eq!(o.step(0, false), Some(B));
    o.focused(B);
    assert_eq!(o.step(HOLD + 1, false), Some(C), "back to the window left");
}
