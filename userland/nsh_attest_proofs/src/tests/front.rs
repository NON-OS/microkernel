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

//! Step 7, `nsh.front`: exactly one capsule holds Network, and it is the
//! release's front, admitted by the vendor's root.

use nsh_attest::steps::{NETWORK, VENDOR};
use nsh_attest::Refusal;

use super::{passes, refused};
use crate::build::parts::{entry, entry_by};
use crate::build::spec::Entry;
use crate::build::Spec;

const FRONT: [u8; 32] = [0xF0; 32];

fn set(extra: Vec<Entry>) -> Refusal {
    let mut entries = vec![entry_by(0, [0xFA; 32], 0, VENDOR), entry(6, [0xA7; 32], 1 << 9)];
    entries.extend(extra);
    refused(Spec { entries, ..Spec::default() })
}

#[test]
fn the_network_bit_is_the_receipt_readers() {
    assert!(attest_receipt::reaches_network(NETWORK));
    assert!(!attest_receipt::reaches_network(!NETWORK));
    passes(Spec::default());
}

#[test]
fn a_second_network_capsule_under_another_authority_is_refused() {
    let front = entry_by(5, FRONT, NETWORK, VENDOR);
    assert_eq!(set(vec![front, entry_by(7, [0x77; 32], NETWORK, 1)]), Refusal::NotVendor);
    assert_eq!(set(vec![front, entry_by(7, [0x77; 32], NETWORK, 255)]), Refusal::NotVendor);
    assert_eq!(set(vec![front, entry_by(7, [0x77; 32], NETWORK, VENDOR)]), Refusal::Front);
}

#[test]
fn a_front_twice_or_not_admitted_by_the_vendor_is_refused() {
    let front = entry_by(5, FRONT, NETWORK, VENDOR);
    assert_eq!(set(vec![front, entry_by(8, FRONT, NETWORK, VENDOR)]), Refusal::Front);
    assert_eq!(set(vec![entry_by(5, FRONT, NETWORK, 1)]), Refusal::NotVendor);
    assert_eq!(set(vec![entry_by(5, [0xF1; 32], NETWORK, VENDOR)]), Refusal::Front);
}

#[test]
fn a_front_without_network_or_no_network_capsule_is_refused() {
    assert_eq!(set(vec![entry_by(5, FRONT, 1 << 3, VENDOR)]), Refusal::Front);
    assert_eq!(set(vec![]), Refusal::Front);
    assert!(
        refused(Spec { entries: vec![entry(0, FRONT, 1 << 3)], ..Spec::default() })
            == Refusal::Front
    );
}
