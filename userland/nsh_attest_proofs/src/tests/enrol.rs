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

//! Step 4, what the certificate names: this fleet, the host the user asked
//! for, this document's AK and these host keys, until `not_after_ms`.

use nsh_attest::Refusal;

use super::{passes, refused};
use crate::build::ak::Key;
use crate::build::Spec;

#[test]
fn a_certificate_for_another_fleet_is_refused() {
    let s = Spec { cert: |c| c.fleet = [0xF2; 16], ..Spec::default() };
    assert_eq!(refused(s), Refusal::EnrolFleet);
}

#[test]
fn a_certificate_for_another_host_is_refused() {
    let s = Spec { cert: |c| c.host = b"db-2.eu.example".to_vec(), ..Spec::default() };
    assert_eq!(refused(s), Refusal::EnrolHost);
    let s = Spec { host: b"db-1.eu.example.".to_vec(), ..Spec::default() };
    assert_eq!(refused(s), Refusal::EnrolHost);
}

#[test]
fn a_certificate_for_another_ak_is_refused() {
    let s = Spec { cert: |c| c.ak = Key::new(2).public(), ..Spec::default() };
    assert_eq!(refused(s), Refusal::EnrolAk);
}

#[test]
fn a_certificate_for_other_host_keys_is_refused() {
    let s = Spec { cert: |c| c.host_keys[63] ^= 1, ..Spec::default() };
    assert_eq!(refused(s), Refusal::EnrolHostKeys);
}

#[test]
fn a_certificate_is_refused_from_its_not_after_on() {
    for now in [10_000, 10_001, u64::MAX] {
        assert_eq!(refused(Spec { now_ms: now, ..Spec::default() }), Refusal::EnrolExpired);
    }
    passes(Spec { now_ms: 9_999, ..Spec::default() });
}

#[test]
fn a_revoked_certificate_is_refused() {
    assert_eq!(refused(Spec { revoke: true, ..Spec::default() }), Refusal::EnrolRevoked);
    let mut b = Spec::default().build();
    b.revoked.push([0x33; 64]);
    assert!(b.check().is_ok(), "another enrolment's revocation is not this one's");
}
