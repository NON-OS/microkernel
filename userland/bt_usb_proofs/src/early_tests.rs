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

//! A peer may read this host's own small database as soon as the link is
//! up, before pairing (a keyboard reads the central's name, Vol 3, Part C,
//! 12.1); it is answered on the clear link, and pairing goes on.

use crate::air::Act;
use crate::fixture::{air, keyboard, link};
use crate::link::End;

#[test]
fn the_device_name_is_answered_before_pairing() {
    let (mut host, ours, mut env) = keyboard();
    // A Read Request for handle 3, the Device Name's value (letter E-17).
    air(&mut host).early_att.push(vec![0x0a, 0x03, 0x00]);
    air(&mut host).script.push_back(Act::Drop(0x13));
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert_eq!(air(&mut host).att_answers, [b"\x0bNONOS".to_vec()]);
    assert!(s.bond.is_some());
}
