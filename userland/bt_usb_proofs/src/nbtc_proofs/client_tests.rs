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

//! What the capsule answers, read with the shared crate's client half, as
//! Settings reads it: both ends of NBTC agree on every op.

use super::client_call::call;
use super::fixture::{board, bonded, KB};
use crate::nbtc::ops::*;
use crate::nbtc::Part;
use nonos_bt_proto as p;

#[test]
fn health_and_link_read_as_the_client_reads_them() {
    let mut b = board();
    let health = p::Health::read(&call(&mut b, OP_HEALTH, &[], 0).1).unwrap();
    assert_eq!(health, p::Health { version: 1, controller: true });
    bonded(&mut b, KB, true);
    b.linked(KB, false);
    b.ready(true, true, false);
    b.scan = true;
    let link = p::Link::read(&call(&mut b, OP_LINK, &[], 0).1).unwrap();
    assert_eq!((link.state, link.place), (READY, 0));
    assert!(link.has(p::Link::KEYBOARD) && link.has(p::Link::MOUSE));
    assert!(link.has(p::Link::SCANNING) && !link.has(p::Link::KEYS_POST));
    assert_eq!(call(&mut b, OP_PAIR, &[], 0).0, E_OK);
    b.ask(Part::Enter);
    let link = p::Link::read(&call(&mut b, OP_LINK, &[], 1).1).unwrap();
    assert!(link.has(p::Link::WINDOW) && link.has(p::Link::WAITING));
}

#[test]
fn each_part_reads_as_it_was_asked() {
    let mut b = board();
    for part in [Part::None, Part::Show(123_456), Part::Enter, Part::Confirm(999_999)] {
        b.ask(part);
        assert_eq!(p::Asked::read(&call(&mut b, OP_PASSKEY, &[], 0).1), Some(part));
    }
}
