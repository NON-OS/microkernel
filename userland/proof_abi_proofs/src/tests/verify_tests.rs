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

//! The verify order over a real device proof bound to a real record. Every
//! step through the publics accepts; the last refuses, since the bytes in the
//! isolation section are not a proof until isolation_stark lands.

use nonos_proof_abi::{verify, Pins, Refusal};

use crate::fixture::{honest, pins, DAY};

#[test]
fn an_honest_container_reaches_the_isolation_proof() {
    let (_, parts, roots) = honest();
    let got = verify(&parts.bytes(), &pins(&[roots]));
    // Header, length or publics: nox_verify's codes 2 to 4, never anything earlier.
    assert!(matches!(got, Err(Refusal::Isolation(2..=4))), "{got:?}");
}

#[test]
fn a_device_proof_for_another_record_is_refused() {
    let (mut f, mut parts, roots) = honest();
    f.record.seal_no += 1;
    parts.set(1, f.record.encode().to_vec());
    assert_eq!(verify(&parts.bytes(), &pins(&[roots])), Err(Refusal::DeviceBinding));
}

#[test]
fn roots_listing_and_days_are_the_verifiers() {
    let (_, parts, roots) = honest();
    let (b, accepted) = (parts.bytes(), [roots]);
    let mut other = roots;
    other.kernel_root[0] ^= 1;
    assert_eq!(verify(&b, &pins(&[other])), Err(Refusal::DeviceTrust));
    let mut p = pins(&accepted);
    p.listing_id[0] ^= 1;
    assert_eq!(verify(&b, &p), Err(Refusal::DeviceTrust));
    for days in [(DAY + 1, DAY + 3), (DAY - 4, DAY + 4), (DAY + 1, DAY)] {
        let p = Pins { days, ..pins(&accepted) };
        assert_eq!(verify(&b, &p), Err(Refusal::DeviceTrust), "{days:?}");
    }
}

#[test]
fn a_tampered_device_proof_is_refused() {
    let (_, mut parts, roots) = honest();
    let mut d = parts.sections[1].1.clone();
    let mid = 192 + (d.len() - 192) / 2;
    d[mid] ^= 0x01;
    parts.set(2, d);
    assert_eq!(verify(&parts.bytes(), &pins(&[roots])), Err(Refusal::DeviceProof));
}
