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

//! A release's network stack, refused before use when it could let a
//! capsule hold Network unnoticed: a zero measurement, one listed twice,
//! the front listed as stack, or more than `MAX_NETWORK`.

use nsh_attest::policy::MAX_NETWORK;
use nsh_attest::{Policy, PolicyError, Release};

use crate::build::keys::{hybrid, AUTHORITY};
use crate::build::parts::release;

fn make(r: Release) -> Result<(), PolicyError> {
    Policy::new([1; 16], vec![hybrid(AUTHORITY).authority()], vec![r], vec![]).map(|_| ())
}

#[test]
fn the_network_stack_is_held_to_its_rules() {
    assert_eq!(make(release()), Ok(()));
    assert_eq!(make(Release { network: vec![], ..release() }), Ok(()), "a front alone");
    let zero = Release { network: vec![[0xE1; 32], [0; 32]], ..release() };
    assert_eq!(make(zero), Err(PolicyError::ZeroValue));
    let twice = Release { network: vec![[0xE1; 32], [0xE1; 32]], ..release() };
    assert_eq!(make(twice), Err(PolicyError::Network));
    let front = Release { network: vec![[0xF0; 32]], ..release() };
    assert_eq!(make(front), Err(PolicyError::Network));
    let full: Vec<[u8; 32]> = (1..=MAX_NETWORK as u8).map(|i| [i; 32]).collect();
    assert_eq!(make(Release { network: full.clone(), ..release() }), Ok(()));
    let over = [full, vec![[0xEE; 32]]].concat();
    assert_eq!(make(Release { network: over, ..release() }), Err(PolicyError::Network));
}
