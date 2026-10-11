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

//! C8v: a positioner takes only what xdg-shell allows.

use crate::wayland_popup::positioner::{set, Positioner};

#[test]
fn a_positioner_refuses_what_xdg_shell_calls_invalid_input() {
    let mut p = Positioner::default();
    assert!(set(&mut p, 1, &[50, 40]) && set(&mut p, 3, &[6]));
    let before = p;
    for (op, w) in [
        (1, vec![0, 5]),
        (1, vec![5, (-1i32) as u32]),
        (2, vec![0, 0, (-1i32) as u32, 1]),
        (3, vec![9]),
        (4, vec![9]),
        (1, vec![5]),
        (10, vec![]),
    ] {
        assert!(!set(&mut p, op, &w), "op {op} {w:?}");
    }
    assert_eq!(p, before, "a refused request changes nothing");
    assert!(set(&mut p, 2, &[0, 0, 0, 0]), "an empty anchor rectangle is allowed");
    assert!(set(&mut p, 7, &[]) && set(&mut p, 8, &[1, 1]) && set(&mut p, 9, &[3]));
}
