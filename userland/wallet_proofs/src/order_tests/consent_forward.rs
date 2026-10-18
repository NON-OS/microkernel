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

//! Under T1b the window manager refuses the wallet's own raise and focus
//! with E_PERM (a store's POST is no input to the wallet). The consent
//! takes that as waiting for focus, not a failure: drawn, unarmed, at the
//! consent, until OP_QUERY_FOCUS says the wallet holds focus; then ARM_MS.

use crate::order::terms::arming::{buy_armed, look, Guard, Look, ARM_MS};
use crate::order::terms::forward::forward;

const T: u64 = 90_000;

#[test]
fn a_raise_and_focus_refused_wait_for_focus_and_arm_a_second_after_it() {
    let refused = forward(Err("wm rejected window_raise"), Err("wm rejected window_focus"));
    assert_eq!(refused, forward(Ok(()), Ok(())), "a refusal is taken as a yes is");
    assert_eq!(refused, Guard::default(), "unarmed, its second not begun");
    let mut g = Guard { drawn: true, ..refused };
    for t in (0..40).map(|i| T + i * 250) {
        g = match look(g, false, t) {
            Look::Held(g) => g,
            Look::Lost => panic!("the consent was left at {t} without focus ever held"),
        };
        assert!(!buy_armed(g, false, t), "armed without focus at {t}");
        assert!(!buy_armed(g, true, t), "focus at the press alone is no second held");
    }
    assert_eq!(g, Guard { drawn: true, since: None }, "still at the drawn consent");
    let focus = T + 10_000;
    let Look::Held(g) = look(g, true, focus) else { panic!("focus arriving is no loss") };
    assert_eq!(g.since, Some(focus), "the second starts when focus arrives");
    assert!(!buy_armed(g, true, focus + ARM_MS - 1));
    assert!(buy_armed(g, true, focus + ARM_MS));
}
