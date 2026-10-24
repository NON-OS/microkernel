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

//! The revocation list: read exactly, ids strictly ascending and never
//! zero, for the table's fleet, signed by one of the table's authorities.

use crate::build::revoke::RevB;
use crate::build::table::TableB;
use crate::build::{HOUR, NOW};
use nsh_roles::{open_policy, open_revocations, PolicyError, RevocationList};
use nsh_wire::WireError;

pub fn open(r: &RevB) -> Result<(), PolicyError> {
    let t = TableB::default();
    let tb = t.bytes();
    let policy = open_policy(&tb, &t.anchor, 0).expect("table");
    open_revocations(&r.bytes(), &policy, 0).map(|_| ())
}

#[test]
fn a_good_list_opens_with_what_it_names() {
    let r = RevB { ids: vec![[1; 16], [2; 16], [0x99; 16]], ..RevB::default() };
    let bytes = r.bytes();
    let list = RevocationList::parse(&bytes).expect("parses");
    assert_eq!((list.issued_ms, list.len(), list.is_empty()), (NOW - HOUR, 3, false));
    assert_eq!(list.ids().collect::<Vec<_>>(), [&[1u8; 16][..], &[2; 16], &[0x99; 16]]);
    assert_eq!(open(&r), Ok(()));
    assert_eq!(open(&RevB { ids: vec![], ..RevB::default() }), Ok(()));
}

#[test]
fn the_preamble_and_the_fleet_are_exact() {
    let good = RevB::default().bytes();
    for (at, v, e) in [(0, b'X', WireError::Magic), (4, 2, WireError::BodyVersion)] {
        let mut b = good.clone();
        b[at] = v;
        assert_eq!(RevocationList::parse(&b).map(|_| ()), Err(e.into()));
    }
    let mut b = good.clone();
    b[7] = 1;
    assert_eq!(RevocationList::parse(&b).map(|_| ()), Err(WireError::Reserved.into()));
    let zero = RevB { fleet: [0; 16], ..RevB::default() };
    assert_eq!(open(&zero), Err(WireError::Value.into()));
    assert_eq!(open(&RevB { fleet: [0xF2; 16], ..RevB::default() }), Err(PolicyError::Fleet));
}

#[test]
fn ids_are_ascending_never_twice_and_never_zero() {
    let ids = |ids: Vec<[u8; 16]>| open(&RevB { ids, ..RevB::default() });
    assert_eq!(ids(vec![[2; 16], [1; 16]]), Err(PolicyError::Order));
    assert_eq!(ids(vec![[2; 16], [2; 16]]), Err(PolicyError::Order));
    assert_eq!(ids(vec![[0; 16], [2; 16]]), Err(PolicyError::Range));
}
