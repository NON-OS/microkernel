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

//! A controller without the feature, or one that refuses, or one that
//! answers out of range: the host asks for nothing, and the link stands.

use super::sim::{asked, linked, DLE, READ};
use crate::sim::stack::opcodes;

#[test]
fn without_the_feature_nothing_is_read_or_asked() {
    let mut s = linked(0, [251, 2120, 251, 2120], 0, None);
    assert!(!opcodes(&mut s).contains(&READ));
    assert!(asked(&mut s).is_none());
}

#[test]
fn a_refused_or_out_of_range_maximum_asks_for_nothing() {
    let mut s = linked(DLE, [251, 2120, 251, 2120], 0, Some((READ, 0x01)));
    assert!(asked(&mut s).is_none(), "Unknown HCI Command");
    for m in [
        [26, 2120, 251, 2120],
        [252, 2120, 251, 2120],
        [251, 327, 251, 2120],
        [251, 2120, 251, 17041],
    ] {
        let mut s = linked(DLE, m, 0, None);
        assert!(asked(&mut s).is_none(), "{m:?}");
    }
}

#[test]
fn a_refused_ask_leaves_the_link_up() {
    // `linked` asserts the connection stands; the refusal is all it gets.
    let mut s = linked(DLE, [251, 2120, 251, 2120], 0x12, None);
    assert!(asked(&mut s).is_some());
}
