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

//! What the host asks a connection for.

use super::sim::{asked, linked, CREATE, DLE, READ, SET};
use crate::sim::stack::opcodes;

#[test]
fn a_controller_with_the_feature_is_asked_for_its_maximum_after_connecting() {
    let mut s = linked(DLE, [251, 2120, 251, 2120], 0, None);
    let o = opcodes(&mut s);
    let (read, create) = (o.iter().position(|&c| c == READ), o.iter().position(|&c| c == CREATE));
    assert!(read.unwrap() < create.unwrap(), "read at bring-up");
    assert_eq!(o.last(), Some(&SET), "asked once the link is up");
    assert_eq!(asked(&mut s), Some(vec![0x40, 0, 251, 0, 0x48, 0x08]));
}

#[test]
fn the_ask_is_the_smaller_of_the_maximum_and_251_octets_in_2120_us() {
    let mut s = linked(DLE, [100, 900, 251, 17040], 0, None);
    assert_eq!(asked(&mut s), Some(vec![0x40, 0, 100, 0, 0x84, 0x03]));
    let mut s = linked(DLE, [251, 17040, 27, 328], 0, None);
    assert_eq!(asked(&mut s), Some(vec![0x40, 0, 251, 0, 0x48, 0x08]));
}
