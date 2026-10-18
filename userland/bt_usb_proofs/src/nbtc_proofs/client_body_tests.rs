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

//! SCAN's, ANSWER's and FORGET's bodies, laid by the shared crate's client
//! half, are what the capsule takes.

use super::client_call::call;
use super::fixture::{board, bonded, KB};
use crate::link::Answer;
use crate::nbtc::ops::*;
use crate::nbtc::Part;
use nonos_bt_proto as p;

#[test]
fn the_client_bodies_are_what_the_capsule_takes() {
    let mut b = board();
    bonded(&mut b, KB, true);
    for on in [false, true] {
        assert_eq!(call(&mut b, OP_SCAN, &p::scan_body(on), 0).0, E_OK);
        assert_eq!(b.scan, on);
    }
    let answers = [
        (Part::Enter, p::Reply::Digits(999_999), Answer::Passkey(999_999)),
        (Part::Confirm(5), p::Reply::Same(false), Answer::Same(false)),
        (Part::Confirm(5), p::Reply::Same(true), Answer::Same(true)),
        (Part::Enter, p::Reply::Decline, Answer::Cancel),
    ];
    for (part, reply, taken) in answers {
        b.ask(part);
        assert_eq!(call(&mut b, OP_ANSWER, &p::answer_body(reply).unwrap(), 0).0, E_OK);
        assert_eq!(b.take_answer(), Some(taken));
    }
    assert_eq!(p::answer_body(p::Reply::Digits(1_000_000)), None, "seven digits");
    assert_eq!(call(&mut b, OP_FORGET, &p::forget_body(KB.0, &KB.1), 0).0, E_OK);
    assert!(b.bonds.book().bonds().is_empty());
}
