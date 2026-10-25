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

//! A good role table opens and says what it was written to say; its
//! preamble, fleet and revocation age are each held to their rule.

use crate::build::keys::{key, K};
use crate::build::table::TableB;
use crate::build::{FLEET, HOUR};
use nsh_roles::{open_policy, PolicyError, Unverified};

pub fn open(t: &TableB) -> Result<(), PolicyError> {
    open_policy(&t.bytes(), &t.anchor, 0).map(|_| ())
}

/// Parse only: for shape refusals, which come before any signature.
pub fn parse(bytes: &[u8]) -> Result<(), PolicyError> {
    Unverified::parse(bytes).map(|_| ())
}

#[test]
fn a_good_table_opens_with_what_it_names() {
    let t = TableB::default();
    let bytes = t.bytes();
    let p = open_policy(&bytes, &t.anchor, 0).expect("opens");
    assert_eq!(p.fleet_id, &FLEET);
    assert_eq!(p.revocation_max_age_ms, 24 * HOUR);
    let ids = |ks: &[K]| {
        let mut v: Vec<_> = ks.iter().map(|k| key(*k).id()).collect();
        v.sort_unstable();
        v
    };
    let got: Vec<_> = p.authorities.iter().map(|k| k.id).collect();
    assert_eq!(got, ids(&[K::AuthA, K::AuthB]));
    let got: Vec<_> = p.approvers.iter().map(|k| k.id).collect();
    assert_eq!(got, ids(&[K::Appr1, K::Appr2, K::Appr3]));
    assert!(p.approvers.iter().all(|k| k.subject == key_subject(&k.id)));
    assert!(p.role(b"operator").is_some() && p.role(b"root").is_none());
}

fn key_subject(id: &[u8; 32]) -> [u8; 64] {
    let all = [K::Appr1, K::Appr2, K::Appr3];
    all.iter().map(|k| key(*k)).find(|k| &k.id() == id).expect("known").subject()
}
