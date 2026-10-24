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

//! A policy's releases and revocations, refused before use when they could
//! admit an unmeasured host or match one kernel twice.

use nsh_attest::policy::{MAX_RELEASES, MAX_REVOKED};
use nsh_attest::{Policy, PolicyError, Release};

use crate::build::keys::{hybrid, AUTHORITY};
use crate::build::parts::release;

fn make(fleet: [u8; 16], n_auth: usize, rel: Vec<Release>, rev: usize) -> Result<(), PolicyError> {
    let a = hybrid(AUTHORITY).authority();
    let auth = (0..n_auth).map(|_| a.clone()).collect();
    Policy::new(fleet, auth, rel, vec![[0; 64]; rev]).map(|_| ())
}

#[test]
fn no_release_or_too_many_is_refused() {
    assert_eq!(make([1; 16], 1, vec![], 0), Err(PolicyError::Releases));
    let many: Vec<_> =
        (0..=MAX_RELEASES as u8).map(|i| Release { pcr9: [i + 1; 32], ..release() }).collect();
    assert_eq!(make([1; 16], 1, many, 0), Err(PolicyError::Releases));
}

#[test]
fn a_release_with_a_zero_value_is_refused() {
    let edits: [fn(&mut Release); 5] = [
        |r| r.pcr8 = [0; 32],
        |r| r.pcr9 = [0; 32],
        |r| r.pcr14 = [0; 32],
        |r| r.loader = [0; 32],
        |r| r.front = [0; 32],
    ];
    for edit in edits {
        let mut r = release();
        edit(&mut r);
        assert_eq!(make([1; 16], 1, vec![r], 0), Err(PolicyError::ZeroValue));
    }
}

#[test]
fn two_releases_sharing_a_pcr9_or_too_many_revocations_are_refused() {
    let twin = Release { version: "0.9.5".into(), ..release() };
    assert_eq!(make([1; 16], 1, vec![release(), twin], 0), Err(PolicyError::SharedPcr9));
    assert_eq!(make([1; 16], 1, vec![release()], MAX_REVOKED + 1), Err(PolicyError::Revoked));
}
