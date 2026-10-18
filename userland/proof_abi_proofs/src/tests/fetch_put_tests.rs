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

//! PUT chunks cover a container in order, each frame within FRAME_MAX, and a
//! chunk that is empty, oversized or runs past its total is refused.

use nonos_proof_abi::wire::fetch::{put_bodies, Put, CHUNK_MAX, FETCH_MAGIC, PUT_HEAD};
use nonos_proof_abi::wire::{request, FRAME_MAX};
use nonos_proof_abi::MAX_BYTES;

#[test]
fn put_bodies_cover_the_container_in_order() {
    let c: Vec<u8> = (0..CHUNK_MAX * 2 + 5).map(|i| i as u8).collect();
    let bodies = put_bodies(&[9; 32], &c);
    assert_eq!(bodies.len(), 3);
    let mut back = Vec::new();
    for (i, b) in bodies.iter().enumerate() {
        assert!(request(FETCH_MAGIC, 1, 1, b).len() <= FRAME_MAX, "a PUT frame fits");
        let p = Put::decode(b).expect("decodes");
        assert_eq!(
            (*p.listing_id, p.total, p.offset as usize),
            ([9; 32], c.len() as u32, back.len())
        );
        assert_eq!(p.last(), i == 2);
        back.extend_from_slice(p.chunk);
    }
    assert_eq!(back, c);
}

#[test]
fn a_put_that_is_empty_oversized_or_past_its_total_is_refused() {
    let put = |total: u32, offset: u32, n: usize| {
        let chunk = vec![1u8; n];
        Put { listing_id: &[2; 32], total, offset, chunk: &chunk }.encode()
    };
    assert!(Put::decode(&put(10, 0, 10)).is_some());
    assert!(Put::decode(&put(10, 0, 0)).is_none(), "empty");
    assert!(Put::decode(&put(10, 1, 10)).is_none(), "past its total");
    assert!(Put::decode(&put(10, u32::MAX, 1)).is_none(), "an offset past its total");
    let max = MAX_BYTES as u32;
    assert!(Put::decode(&put(max + 1, 0, 1)).is_none(), "over 1 MiB");
    assert!(Put::decode(&put(max, 0, CHUNK_MAX + 1)).is_none(), "an oversized chunk");
    assert!(Put::decode(&put(10, 0, 10)[..PUT_HEAD - 1]).is_none(), "short");
}
