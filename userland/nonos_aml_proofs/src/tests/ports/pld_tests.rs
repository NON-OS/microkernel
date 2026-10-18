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

//! _PLD and _UPC read field by field, and every buffer too short for its
//! revision, or of revision 0, refused whole rather than read in part.

use super::rig::pld_buf;
use crate::object::Object;
use crate::ports::{pld, upc, Panel};

fn buf(b: Vec<u8>) -> Object {
    Object::package(vec![Object::buffer(b)])
}

#[test]
fn fields_and_lengths() {
    let p = pld(&buf(pld_buf(2, 20, true, true, 5))).unwrap();
    assert_eq!(
        (p.revision, p.user_visible, p.lid, p.dock, p.panel),
        (2, true, true, false, Panel::Back)
    );
    assert!(pld(&buf(pld_buf(1, 16, false, false, 7))).is_some_and(|p| p.panel == Panel::Unknown));
    assert!(pld(&Object::buffer(pld_buf(1, 16, true, false, 0))).is_some(), "a bare buffer");
    for (rev, len) in [(1, 15), (2, 19), (2, 16), (1, 0), (2, 8)] {
        assert_eq!(
            pld(&buf(pld_buf(rev, len, true, false, 0))),
            None,
            "revision {rev}, {len} bytes"
        );
    }
    assert_eq!(pld(&buf(vec![0x80; 20])), None, "revision 0");
    assert_eq!(pld(&Object::package(vec![])), None);
    assert_eq!(pld(&Object::Integer(1)), None);
}

#[test]
fn upc_needs_two_integers_and_a_byte_type() {
    let p = |e: Vec<Object>| upc(&Object::package(e));
    assert_eq!(
        p(vec![Object::Integer(1), Object::Integer(0xFF)]).map(|u| (u.connectable, u.connector)),
        Some((true, 0xFF))
    );
    assert_eq!(p(vec![Object::Integer(1)]), None);
    assert_eq!(p(vec![Object::Integer(1), Object::Integer(0x100)]), None);
    assert_eq!(p(vec![Object::Integer(1), Object::buffer(vec![0])]), None);
}
