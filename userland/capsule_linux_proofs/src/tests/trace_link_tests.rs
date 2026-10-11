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

//! The diagnostic chain's link: BLAKE3 over its domain, the head, the call,
//! its label and errno, in that order; a failed call is labelled with its
//! errno, an unserved one as ENOSYS whatever it returned.

use crate::trace_link::{label, link, DOMAIN, FAILED, SERVED, UNSERVED};

#[test]
fn a_link_is_blake3_over_its_fields_in_order() {
    let head = [0x11u8; 32];
    let want = blake3::hash(
        &[DOMAIN, &head, &60u64.to_le_bytes(), &[FAILED], &9u16.to_le_bytes()].concat(),
    );
    assert_eq!(link(&head, 60, FAILED, 9), *want.as_bytes());
    assert_eq!(DOMAIN, b"NONOS-LINUX-DIAG-v1");
}

#[test]
fn every_field_and_the_order_of_calls_moves_the_head() {
    let z = [0u8; 32];
    let base = link(&z, 1, SERVED, 0);
    assert_ne!(link(&[1; 32], 1, SERVED, 0), base, "another head");
    assert_ne!(link(&z, 2, SERVED, 0), base, "another call");
    assert_ne!(link(&z, 1, FAILED, 0), base, "another label");
    assert_ne!(link(&z, 1, SERVED, 1), base, "another errno");
    let ab = link(&link(&z, 1, SERVED, 0), 2, SERVED, 0);
    let ba = link(&link(&z, 2, SERVED, 0), 1, SERVED, 0);
    assert_ne!(ab, ba, "a chain, not a set");
}

#[test]
fn a_result_is_labelled_as_linux_reads_it() {
    assert_eq!(label(0, false), (SERVED, 0));
    assert_eq!(label(0x7f00_0000_1000, false), (SERVED, 0), "an address");
    assert_eq!(label((-2i64) as u64, false), (FAILED, 2));
    assert_eq!(label((-4095i64) as u64, false), (FAILED, 4095));
    assert_eq!(label((-4096i64) as u64, false), (SERVED, 0), "not an errno");
    assert_eq!(label((-38i64) as u64, true), (UNSERVED, 38));
    assert_eq!(label(0, true), (UNSERVED, 38), "unserved, whatever came back");
}
