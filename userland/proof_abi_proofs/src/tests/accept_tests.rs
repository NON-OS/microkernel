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

//! The whole order accepts the honest container, and its last step is a
//! real isolation proof: it holds to its own bytes and the pinned image.

use nonos_proof_abi::{record_id, tag, verify, Kind, Refusal};
use nox_verify::statements::ATTEST;

use crate::fixture::{expected, honest, DAY};

#[test]
fn an_honest_container_is_accepted() {
    let (f, parts, pinned) = honest();
    let pins = pinned.trust();
    let v = verify(&parts.bytes(), &expected(&f.policy, &pins)).expect("accepted");
    assert_eq!((v.kind, v.window), (Kind::Isolation, DAY));
    assert_eq!(v.declared, 4);
    assert_eq!(v.record_id, record_id(&f.record.encode()));
    assert_eq!(v.app_id, f.record.app_id);
}

#[test]
fn a_flipped_isolation_byte_is_refused() {
    let (f, mut parts, pinned) = honest();
    let pins = pinned.trust();
    let mut iso = parts.sections.iter().find(|s| s.0 == tag::ISOLATION).unwrap().1.clone();
    let mid = iso.len() / 2;
    iso[mid] ^= 1;
    parts.set(tag::ISOLATION, iso);
    let got = verify(&parts.bytes(), &expected(&f.policy, &pins));
    assert!(matches!(got, Err(Refusal::Isolation(_))), "{got:?}");
}

#[test]
fn another_pinned_statement_refuses_the_proof() {
    let (f, parts, pinned) = honest();
    let mut pins = pinned.trust();
    pins.isolation = &ATTEST;
    let got = verify(&parts.bytes(), &expected(&f.policy, &pins));
    assert!(matches!(got, Err(Refusal::Isolation(_))), "{got:?}");
}
