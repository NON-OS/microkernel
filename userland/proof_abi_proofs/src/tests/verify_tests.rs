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

//! The verify order over a real device proof bound to a real record and a
//! real isolation proof over its histogram.

use nonos_proof_abi::{tag, verify, Expected, Refusal};

use crate::fixture::{expected, honest, DAY};

#[test]
fn a_device_proof_for_another_record_is_refused() {
    let (mut f, mut parts, pinned) = honest();
    f.record.seal_no += 1;
    parts.set(tag::RECORD, f.record.encode().to_vec());
    let pins = pinned.trust();
    let got = verify(&parts.bytes(), &expected(&f.policy, &pins));
    assert_eq!(got, Err(Refusal::DeviceBinding));
}

#[test]
fn the_window_must_be_a_day_the_verifier_asked_about() {
    let (f, mut parts, pinned) = honest();
    let pins = pinned.trust();
    let b = parts.bytes();
    for days in [(DAY + 1, DAY + 3), (DAY - 4, DAY + 4), (DAY + 1, DAY)] {
        let ex = Expected { days, ..expected(&f.policy, &pins) };
        assert_eq!(verify(&b, &ex), Err(Refusal::Window), "{days:?}");
    }
    parts.window = DAY + 1; // a day the scope was not made for
    let got = verify(&parts.bytes(), &expected(&f.policy, &pins));
    assert_eq!(got, Err(Refusal::DeviceTrust));
}

#[test]
fn roots_registry_life_and_listing_are_the_verifiers() {
    let (f, parts, pinned) = honest();
    let b = parts.bytes();
    let mut kernel = pinned.clone();
    kernel.kernel_root[0] ^= 1;
    let mut expired = pinned.clone();
    expired.registrars[0].life_days = 10;
    let mut depth = pinned.clone();
    depth.registrars[0].depth += 1;
    for p in [kernel, expired, depth] {
        let pins = p.trust();
        assert_eq!(verify(&b, &expected(&f.policy, &pins)), Err(Refusal::DeviceTrust));
    }
    let pins = pinned.trust();
    let ex = Expected { listing_id: [0x4D; 32], ..expected(&f.policy, &pins) };
    assert_eq!(verify(&b, &ex), Err(Refusal::DeviceTrust), "another listing's scope");
}
