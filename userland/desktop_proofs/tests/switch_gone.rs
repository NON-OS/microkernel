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
//! A window gone in the middle of a run leaves the strip, and the next
//! press lands where a press past it would have (state/ctx/switch forget.rs).

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
fn switch_gone_forward_lands_past_it() {
    let mut o = four();
    assert_eq!(o.step(0, false), Some(C));
    o.closed(C);
    assert_eq!(on(&o).0, [D, B, A]);
    assert_eq!(o.step(10, false), Some(B));
}

#[test]
fn switch_gone_back_lands_past_it() {
    let mut o = four();
    assert_eq!(o.step(0, true), Some(A));
    o.closed(A);
    assert_eq!(o.step(10, true), Some(B));
    let mut o = four();
    assert_eq!(o.step(0, true), Some(A));
    assert_eq!(o.step(1, true), Some(B));
    o.closed(B);
    assert_eq!(o.step(2, true), Some(C));
    assert_eq!(o.step(3, false), Some(A), "and on the way forward again");
}
