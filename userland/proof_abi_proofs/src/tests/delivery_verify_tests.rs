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

//! A delivery verifies for its buyer and says what it attests; it never
//! passes as an isolation proof, nor one as it; and a record that is final,
//! not vendor, or another app's is refused before the proof is read.

use nonos_proof_abi::{verify, verify_delivery, Record, Refusal};

use crate::fixture::{delivered, delivery_expected, expected, honest, DAY};

#[test]
fn a_delivery_verifies_for_its_buyer() {
    let d = delivered();
    let pinned = d.pinned();
    let pins = pinned.trust();
    let v = verify_delivery(&d.bytes(), &delivery_expected(&d.policy, &pins)).expect("accepted");
    assert_eq!((v.window, v.service_meas, v.from, v.to), (DAY, d.record.sup_meas, 100, 200));
    assert_eq!(v.record_id, d.body.record_id);
}

#[test]
fn neither_kind_passes_as_the_other() {
    let d = delivered();
    let pinned = d.pinned();
    let pins = pinned.trust();
    assert_eq!(verify(&d.bytes(), &expected(&d.policy, &pins)).err(), Some(Refusal::Kind));
    let (f, parts, hp) = honest();
    let pins = hp.trust();
    let ex = delivery_expected(&f.policy, &pins);
    assert_eq!(verify_delivery(&parts.bytes(), &ex).err(), Some(Refusal::Kind));
}

#[test]
fn a_record_that_is_final_foreign_or_another_apps_is_refused() {
    let d = delivered();
    let pinned = d.pinned();
    let pins = pinned.trust();
    let ex = delivery_expected(&d.policy, &pins);
    let with = |r: Record| d.bytes_with(&r.encode(), &d.body.encode(), &d.meas.concat());
    let refused = |r: Record| verify_delivery(&with(r), &ex).err();
    assert_eq!(refused(Record { flags: 1, ..d.record }), Some(Refusal::Supervisor), "final");
    assert_eq!(refused(Record { flags: 2, ..d.record }), Some(Refusal::Supervisor), "saturated");
    assert_eq!(refused(Record { authority: 1, ..d.record }), Some(Refusal::Supervisor));
    assert_eq!(refused(Record { sup_meas: [1; 32], ..d.record }), Some(Refusal::Supervisor));
    assert_eq!(refused(Record { app_id: [9; 32], ..d.record }), Some(Refusal::AppId));
}
