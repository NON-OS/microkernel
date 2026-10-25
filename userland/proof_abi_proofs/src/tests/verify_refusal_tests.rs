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

//! The record's sealer, then the steps after the device proof, each refusing
//! with its own reason.

use nonos_proof_abi::{tag, verify, Refusal};

use crate::fixture::{expected, honest, policy, LISTING};

#[test]
fn only_a_pinned_vendor_supervisor_seals_what_verifies() {
    let (f, parts, pinned) = honest();
    let b = parts.bytes();
    let mut other = pinned.clone();
    other.supervisors = vec![[0x5D; 32]];
    let pins = other.trust();
    assert_eq!(verify(&b, &expected(&f.policy, &pins)), Err(Refusal::Supervisor));
}

#[test]
fn the_measurements_must_rebuild_m_and_name_the_program() {
    let (f, mut parts, pinned) = honest();
    let pins = pinned.trust();
    let swapped: Vec<u8> = f.meas.iter().rev().flatten().copied().collect();
    for list in [swapped, f.meas[0].to_vec()] {
        parts.set(tag::MEASUREMENTS, list);
        let got = verify(&parts.bytes(), &expected(&f.policy, &pins));
        assert_eq!(got, Err(Refusal::Measurements));
    }
}

#[test]
fn the_policy_must_be_the_releases_and_name_the_records_app() {
    let (_, mut parts, pinned) = honest();
    let pins = pinned.trust();
    let other = policy(&LISTING, &[0, 1]); // the same program, another set
    assert_eq!(verify(&parts.bytes(), &expected(&other, &pins)), Err(Refusal::Policy));
    parts.set(tag::POLICY, other.to_vec());
    assert_eq!(verify(&parts.bytes(), &expected(&other, &pins)), Err(Refusal::AppId));
    let vendor = policy(&[0; 32], &[0, 1, 60, 231]); // ids zero, not in the table
    parts.set(tag::POLICY, vendor.to_vec());
    assert_eq!(verify(&parts.bytes(), &expected(&vendor, &pins)), Err(Refusal::Policy));
}

#[test]
fn the_publics_must_carry_the_records_hc_and_the_policys_a() {
    let (f, mut parts, pinned) = honest();
    let pins = pinned.trust();
    for word in [0, 5] {
        let mut p = f.publics;
        p[word] ^= 1;
        parts.set(tag::PUBLICS, p.iter().flat_map(|w| w.to_le_bytes()).collect());
        let got = verify(&parts.bytes(), &expected(&f.policy, &pins));
        assert_eq!(got, Err(Refusal::Publics), "word {word}");
    }
}
