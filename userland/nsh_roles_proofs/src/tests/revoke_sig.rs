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

//! A list names at most 4096 grants, and is signed by one of the table's
//! authorities with both halves of the signature verifying.

use super::revoke::open;
use crate::build::keys::{key, K};
use crate::build::revoke::RevB;
use crate::build::Spoil;
use nsh_roles::{PolicyError, RevocationList, MAX_REVOKED};

#[test]
fn a_list_names_at_most_4096_ids() {
    let many = |n: usize| (1..=n as u32).map(id).collect::<Vec<_>>();
    assert_eq!(open(&RevB { ids: many(MAX_REVOKED), ..RevB::default() }), Ok(()));
    let r = RevB { ids: many(MAX_REVOKED + 1), ..RevB::default() };
    assert_eq!(RevocationList::parse(&r.bytes()).map(|_| ()), Err(PolicyError::Range));
}

fn id(i: u32) -> [u8; 16] {
    let mut id = [0xAB; 16];
    id[..4].copy_from_slice(&i.to_be_bytes());
    id
}

#[test]
fn the_signer_is_an_authority_and_both_halves_verify() {
    let appr = RevB { signer: K::Appr1, signer_id: key(K::Appr1).id(), ..RevB::default() };
    assert_eq!(open(&appr), Err(PolicyError::Signer));
    assert_eq!(open(&RevB { signer: K::AuthA, ..RevB::default() }), Err(PolicyError::Signature));
    for spoil in [Spoil::Ed, Spoil::Ml] {
        assert_eq!(open(&RevB { spoil, ..RevB::default() }), Err(PolicyError::Signature));
    }
    let by_a = RevB { signer: K::AuthA, signer_id: key(K::AuthA).id(), ..RevB::default() };
    assert_eq!(open(&by_a), Ok(()));
}
