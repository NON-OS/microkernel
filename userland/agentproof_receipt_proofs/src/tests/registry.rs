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
//! A whole receipt verifies, and says what it carries.

//! What ran: the receipt carries the registry's entries, they must fold to
//! the root the quote signed, and the verdict names each that held Network.

use agentproof_receipt::napr::read::{parse, Fault};
use agentproof_receipt::napr::write;
use agentproof_receipt::verify;
use nsh_attest::steps::NETWORK;

use crate::build::parts::entry_by;
use crate::fixture::Fixture;

/// The fixture's receipt with `entries` in place of the ones its doc signed.
fn with_entries(f: &mut Fixture, entries: &[u8]) -> Vec<u8> {
    let honest = f.bytes();
    let r = parse(&honest).unwrap();
    write::finish(r.bound.to_vec(), r.doc, entries, r.capsules)
}

#[test]
fn the_verdict_names_every_capsule_and_each_that_held_network() {
    let mut f = Fixture::new(b"strict");
    let (bytes, trust) = (f.bytes(), f.trust());
    let v = verify(&bytes, &trust).expect("verifies");
    assert_eq!(v.ran.len(), f.spec.entries.len());
    let net: Vec<u32> = v.ran.iter().filter(|r| r.network()).map(|r| r.pid).collect();
    assert_eq!(net, vec![5], "the fixture's one Network holder, pid 5");
    assert_eq!(v.ran[1].caps & NETWORK, NETWORK);
}

#[test]
fn entries_that_do_not_fold_to_the_signed_root_are_refused() {
    let mut f = Fixture::new(b"strict");
    let trust = f.trust();
    let mut hidden = f.spec.entries.clone();
    hidden.retain(|e| u64::from_be_bytes(e[36..44].try_into().unwrap()) & NETWORK == 0);
    for entries in
        [hidden.concat(), [f.spec.entries.concat(), entry_by(9, [9; 32], 0, 0).to_vec()].concat()]
    {
        let b = with_entries(&mut f, &entries);
        assert_eq!(
            verify(&b, &trust).unwrap_err().name(),
            "registry",
            "a Network holder hidden, or one added"
        );
    }
}

#[test]
fn an_entries_field_off_its_width_or_empty_is_refused_as_entries() {
    let mut f = Fixture::new(b"strict");
    for entries in [vec![], vec![0u8; 44], vec![0u8; 46]] {
        assert_eq!(parse(&with_entries(&mut f, &entries)), Err(Fault::Field("entries")));
    }
}
