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
//! A window gone in the middle of a run that is not the run's own keeps the
//! run where it is, and a gone last window wraps (state/ctx/switch forget.rs).

use desktop_proofs::shell_switch::{SwitchOrder, Win};

const A: Win = (10, 1);
const B: Win = (11, 1);
const C: Win = (12, 1);
const D: Win = (13, 1);

/// The order D, C, B, A: D has focus.
fn four() -> SwitchOrder {
    let mut o = SwitchOrder::default();
    for w in [A, B, C, D] {
        o.focused(w);
    }
    o
}

fn on(o: &SwitchOrder) -> (Vec<Win>, Win) {
    let (w, at) = o.session.as_ref().expect("a run").shown();
    (w.to_vec(), w[at])
}

#[test]
fn switch_gone_elsewhere_keeps_the_run_on_its_window() {
    let mut o = four();
    o.step(0, false);
    o.step(1, false);
    assert_eq!(on(&o).1, B);
    o.closed(D);
    assert_eq!(on(&o), (vec![C, B, A], B), "one before it");
    o.closed(A);
    assert_eq!(on(&o), (vec![C, B], B), "one after it");
}

#[test]
fn switch_gone_last_forward_wraps() {
    let mut o = four();
    for t in 0..3 {
        o.step(t, false);
    }
    assert_eq!(on(&o).1, A);
    o.closed(A);
    assert_eq!(o.step(10, false), Some(D), "past the end is the start");
    o.closed(D);
    assert_eq!(o.step(11, false), Some(C), "and past the start, the next");
}
