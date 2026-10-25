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

//! What is not an isolation container, not a final unsaturated record, or
//! not the device proof it claims to be.

use nonos_proof_abi::{encode, tag, verify, Kind, Refusal, FLAG_FINAL, FLAG_SATURATED};

use crate::fixture::{expected, honest};

#[test]
fn only_isolation_containers_are_checked_here() {
    let (f, _, pinned) = honest();
    let pins = pinned.trust();
    let attest = encode(Kind::Authenticity, 1, &[(tag::ATTEST, b"NATTV4")]).expect("kind 3");
    assert_eq!(verify(&attest, &expected(&f.policy, &pins)), Err(Refusal::Kind));
    assert_eq!(verify(b"NPRF", &expected(&f.policy, &pins)), Err(Refusal::Format));
}

#[test]
fn a_record_that_is_not_final_or_is_saturated_is_refused() {
    let (mut f, mut parts, pinned) = honest();
    let pins = pinned.trust();
    for flags in [0, FLAG_FINAL | FLAG_SATURATED] {
        f.record.flags = flags;
        parts.set(tag::RECORD, f.record.encode().to_vec());
        let got = verify(&parts.bytes(), &expected(&f.policy, &pins));
        assert_eq!(got, Err(Refusal::Supervisor), "flags {flags}");
    }
}

#[test]
fn a_tampered_device_proof_is_refused() {
    let (f, mut parts, pinned) = honest();
    let mut d = parts.sections[1].1.clone();
    let mid = 192 + (d.len() - 192) / 2;
    d[mid] ^= 0x01;
    parts.set(tag::DEVICE, d);
    let pins = pinned.trust();
    let got = verify(&parts.bytes(), &expected(&f.policy, &pins));
    assert_eq!(got, Err(Refusal::DeviceProof));
}
