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

//! load_store's refusals: each answers EINVAL and never reaches mk_capsule_load.

use crate::alloc_watch::largest_during;
use crate::frame::{einval, frame, store};
use crate::payloads::{head, store_payload};
use crate::protocol::OP_LOAD_FROM_STORE;
use crate::syscall;

const HEAD_LEN: usize = 28;
const LENS: [u32; 5] = [3, 5, 7, 11, 13];

/// EINVAL, no load, and no allocation beyond the 8-byte reply: nothing is
/// sized from the lengths the head claims.
fn refused(payload: &[u8]) {
    syscall::reset(1);
    let f = frame(OP_LOAD_FROM_STORE, payload);
    let (out, largest) = largest_during(|| store(&f));
    assert_eq!(out, einval(), "payload of {} bytes", payload.len());
    assert!(syscall::take().is_empty(), "mk_capsule_load was called");
    assert!(largest <= 8, "allocated {largest} bytes on a refusal");
}

#[test]
fn head_shorter_than_head_len_is_refused() {
    for n in [0, HEAD_LEN - 1] {
        refused(&vec![0; n]);
    }
}

#[test]
fn each_length_alone_at_u32_max_is_refused_by_the_total_check() {
    for i in 0..5 {
        let mut lens = [0; 5];
        lens[i] = u32::MAX;
        refused(&head(0, lens));
        let mut p = head(0, lens);
        p.extend_from_slice(&[0; 64]);
        refused(&p);
    }
}

#[test]
fn all_lengths_at_u32_max_are_refused_by_the_total_check() {
    refused(&head(u64::MAX, [u32::MAX; 5]));
}

#[test]
fn total_one_byte_short_is_refused() {
    let mut p = store_payload(9, LENS);
    p.pop();
    refused(&p);
}

#[test]
fn total_one_byte_long_is_refused() {
    let mut p = store_payload(9, LENS);
    p.push(0);
    refused(&p);
}
