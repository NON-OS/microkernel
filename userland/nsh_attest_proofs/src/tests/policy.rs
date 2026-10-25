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

//! A policy that could never admit a host, or could admit an unmeasured
//! one, is refused before it is used.

use nsh_attest::policy::{MAX_AUTHORITIES, MAX_REVOKED};
use nsh_attest::{Authority, Policy, PolicyError, Release};

use crate::build::keys::{hybrid, AUTHORITY, STRANGER};
use crate::build::parts::release;

fn make(fleet: [u8; 16], n_auth: usize, rel: Vec<Release>, rev: usize) -> Result<(), PolicyError> {
    let a = hybrid(AUTHORITY).authority();
    let auth = (0..n_auth).map(|_| a.clone()).collect();
    Policy::new(fleet, auth, rel, vec![[0; 64]; rev]).map(|_| ())
}

#[test]
fn a_good_policy_is_built() {
    assert_eq!(make([1; 16], 1, vec![release()], MAX_REVOKED), Ok(()));
    let auth = vec![hybrid(AUTHORITY).authority(), hybrid(STRANGER).authority()];
    assert!(Policy::new([1; 16], auth, vec![release()], vec![]).is_ok());
}

#[test]
fn a_zero_fleet_is_refused() {
    assert_eq!(make([0; 16], 1, vec![release()], 0), Err(PolicyError::Fleet));
}

#[test]
fn no_authority_too_many_or_one_twice_is_refused() {
    assert_eq!(make([1; 16], 0, vec![release()], 0), Err(PolicyError::Authorities));
    assert_eq!(make([1; 16], 2, vec![release()], 0), Err(PolicyError::Authorities));
    let keys = |n: u8| (0..n).map(|i| Authority::new([i; 32], Box::new([0; 1952]))).collect();
    let r = |n: usize| Policy::new([1; 16], keys(n as u8), vec![release()], vec![]).map(|_| ());
    assert_eq!(r(MAX_AUTHORITIES), Ok(()));
    assert_eq!(r(MAX_AUTHORITIES + 1), Err(PolicyError::Authorities));
}
