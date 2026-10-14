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
//! A link that was up and went down is joined again: within a bound, a
//! The mapping from a join's result code to the step it is, the one rule
//! net.core's join.rs and its rejoin both use (`machine/codes.rs`), over
//! every code a driver sends and the rest of i32 near them.

use crate::autojoin_machine::{step_for, Step};

#[test]
fn every_join_code_maps_to_its_step() {
    let named = [-101, -38, i32::MIN, i32::MAX];
    for code in (-64..=64).chain(named) {
        let want = match code {
            0 => Step::Answered { index: 3, joined: true },
            -11..=-6 | -13 => Step::Refused(3),
            _ => Step::Answered { index: 3, joined: false },
        };
        assert_eq!(step_for(3, code), want, "code {code}");
    }
    // The ones the drivers name, said once more in words.
    assert_eq!(step_for(0, -2), Step::Answered { index: 0, joined: false }, "not heard");
    assert_eq!(step_for(0, -6), Step::Refused(0), "the handshake did not finish");
    assert_eq!(step_for(0, -12), Step::Answered { index: 0, joined: false }, "no randomness");
    assert_eq!(step_for(0, -13), Step::Refused(0), "the person left");
}
