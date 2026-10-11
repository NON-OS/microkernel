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

//! The encoders' length words are checked, never cut down: a container
//! past 1 MiB has no PUT bodies, the largest one has them all, and a PUT
//! at the top of the u32 range is not its own last chunk by wrapping.

use nonos_proof_abi::wire::fetch::{put_bodies, Put, CHUNK_MAX};
use nonos_proof_abi::wire::{reply, request, Header, PROVE_MAGIC};
use nonos_proof_abi::MAX_BYTES;

#[test]
fn a_container_past_the_cap_has_no_bodies_and_the_cap_has_all() {
    assert!(put_bodies(&[1; 32], &vec![0; MAX_BYTES + 1]).is_empty());
    assert!(put_bodies(&[1; 32], &[]).is_empty());
    let all = put_bodies(&[1; 32], &vec![7; MAX_BYTES]);
    assert_eq!(all.len(), MAX_BYTES.div_ceil(CHUNK_MAX));
    let last = Put::decode(all.last().expect("one")).expect("decodes");
    assert_eq!(last.total as usize, MAX_BYTES);
    assert!(last.last());
}

#[test]
fn a_put_at_the_top_of_the_range_is_not_last_by_wrapping() {
    let p = Put { listing_id: &[0; 32], total: 0, offset: u32::MAX, chunk: &[1] };
    assert!(!p.last());
    let p = Put { listing_id: &[0; 32], total: u32::MAX, offset: u32::MAX - 1, chunk: &[1] };
    assert!(p.last());
}

#[test]
fn a_frames_length_word_is_its_payload() {
    let f = request(PROVE_MAGIC, 2, 9, b"abcd");
    let (h, body) = Header::parse(&f, PROVE_MAGIC).expect("parses");
    assert_eq!((h.len, body), (4, &b"abcd"[..]));
    let r = reply(PROVE_MAGIC, &h, -3, b"xy");
    assert_eq!(Header::parse(&r, PROVE_MAGIC).map(|(h, b)| (h.len, b.len())), Some((6, 6)));
}
