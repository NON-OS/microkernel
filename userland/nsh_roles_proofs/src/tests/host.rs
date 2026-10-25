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

//! A host checks with a list its own table trusts: a list opened against
//! another table is refused, and so is one from another fleet.

use crate::build::keys::{key, K};
use crate::build::revoke::RevB;
use crate::build::table::TableB;
use crate::build::NOW;
use nsh_roles::{open_policy, open_revocations, Host, PolicyError};

#[test]
fn a_list_from_another_table_is_refused() {
    let ours = TableB::default();
    let other_fleet = [0xF2; 16];
    let theirs = TableB { fleet: other_fleet, authorities: vec![K::AuthA], ..TableB::default() };
    let (ob, tb) = (ours.bytes(), theirs.bytes());
    let ours = open_policy(&ob, &ours.anchor, 0).expect("ours");
    let theirs = open_policy(&tb, &theirs.anchor, 0).expect("theirs");
    let foreign = RevB { fleet: other_fleet, signer: K::AuthA, ..RevB::default() };
    let foreign = RevB { signer_id: key(K::AuthA).id(), ..foreign };
    let fb = foreign.bytes();
    let list = open_revocations(&fb, &theirs, 0).expect("theirs trusts it");
    assert_eq!(Host::new(&ours, &list, NOW).err(), Some(PolicyError::Fleet));
}

#[test]
fn a_list_signed_by_a_key_this_table_does_not_trust_is_refused() {
    let ours = TableB { authorities: vec![K::AuthA], ..TableB::default() };
    let wider = TableB::default();
    let (ob, wb) = (ours.bytes(), wider.bytes());
    let ours = open_policy(&ob, &ours.anchor, 0).expect("ours");
    let wider = open_policy(&wb, &wider.anchor, 0).expect("wider");
    let rb = RevB::default().bytes();
    let list = open_revocations(&rb, &wider, 0).expect("signed by AuthB");
    assert_eq!(Host::new(&ours, &list, NOW).err(), Some(PolicyError::Signer));
    assert!(Host::new(&wider, &list, NOW).is_ok());
}
