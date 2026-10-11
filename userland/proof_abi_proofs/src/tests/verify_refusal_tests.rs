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

//! The steps after the device proof, each refusing with its own reason.

use nonos_proof_abi::{tag, verify, Kind, Refusal};

use crate::fixture::{honest, pins};

#[test]
fn the_measurements_must_rebuild_m_and_name_the_program() {
    let (f, mut parts, roots) = honest();
    let swapped: Vec<u8> = f.meas.iter().rev().flatten().copied().collect();
    let short = f.meas[0].to_vec();
    for list in [swapped, short] {
        parts.set(tag::MEASUREMENTS, list);
        assert_eq!(verify(&parts.bytes(), &pins(&[roots])), Err(Refusal::Measurements));
    }
}

#[test]
fn the_policy_must_read_and_be_the_records_app() {
    let (f, mut parts, roots) = honest();
    let mut bad = f.policy;
    bad[0] = b'X';
    parts.set(tag::POLICY, bad.to_vec());
    assert_eq!(verify(&parts.bytes(), &pins(&[roots])), Err(Refusal::Policy));
    let mut other = f.policy;
    other[48] ^= 1; // another mask: same program, another app
    parts.set(tag::POLICY, other.to_vec());
    assert_eq!(verify(&parts.bytes(), &pins(&[roots])), Err(Refusal::AppId));
}

#[test]
fn the_publics_must_carry_the_records_hc_and_the_policys_a() {
    let (f, mut parts, roots) = honest();
    for word in [0, 5] {
        let mut p = f.publics;
        p[word] ^= 1;
        let bytes: Vec<u8> = p.iter().flat_map(|w| w.to_le_bytes()).collect();
        parts.set(tag::PUBLICS, bytes);
        assert_eq!(verify(&parts.bytes(), &pins(&[roots])), Err(Refusal::Publics), "word {word}");
    }
}

#[test]
fn only_isolation_containers_are_checked_and_sections_are_required() {
    let (_, mut parts, roots) = honest();
    parts.kind = Kind::Delivery;
    assert_eq!(verify(&parts.bytes(), &pins(&[roots])), Err(Refusal::Kind));
    parts.kind = Kind::Isolation;
    parts.sections.retain(|s| s.0 != tag::MEASUREMENTS);
    assert_eq!(verify(&parts.bytes(), &pins(&[roots])), Err(Refusal::Format));
    assert_eq!(verify(b"NPRF", &pins(&[roots])), Err(Refusal::Format));
}

#[test]
fn a_section_the_order_does_not_check_is_refused() {
    let (_, mut parts, roots) = honest();
    parts.sections.insert(4, (tag::ATTEST, b"NATTV4 unchecked".to_vec()));
    assert_eq!(verify(&parts.bytes(), &pins(&[roots])), Err(Refusal::Format));
}
