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


//! The second form of a repliable message, the one the network's requesters
//! read today: reply blocks counted in a u16, their hops and key rotation
//! named, each block one seed per hop. Checked against the reference crates
//! by the interop oracle; held here offline.

use crate::message::{repliable_additional_surbs, repliable_data, TAG_ADDITIONAL_SURBS, TAG_DATA};
use crate::sphinx::constants::{HEADER_SIZE, NODE_ADDRESS_LENGTH, PAYLOAD_KEY_SEED_SIZE};

/// A reply block of `hops` seeds, as net.nym serialises one.
fn block(hops: usize, fill: u8) -> Vec<u8> {
    vec![fill; 16 + HEADER_SIZE + NODE_ADDRESS_LENGTH + hops * PAYLOAD_KEY_SEED_SIZE]
}

#[test]
fn a_request_names_its_blocks_count_hops_and_rotation_then_carries_them_and_itself() {
    let tag = [0x11; 16];
    let blocks: Vec<Vec<u8>> = (0..24).map(|i| block(4, i as u8)).collect();
    let out = repliable_data(&tag, &blocks, b"req");
    assert_eq!(out[0], 1, "repliable");
    assert_eq!(&out[1..17], &tag);
    assert_eq!(out[17], TAG_DATA);
    assert_eq!(TAG_DATA, 3, "the second form's data tag");
    assert_eq!(u16::from_be_bytes([out[18], out[19]]), 24);
    assert_eq!(out[20], 4, "hops, counted from the block's seeds");
    assert_eq!(out[21], 0, "rotation unknown: a hop tries both keys it holds");
    let body = &out[22..];
    assert_eq!(body.len(), 24 * blocks[0].len() + 3);
    assert_eq!(&body[..blocks[0].len()], &blocks[0][..]);
    assert_eq!(&body[body.len() - 3..], b"req");
}

#[test]
fn a_seed_block_is_well_under_half_a_full_key_block() {
    // Four full 192-byte keys made a block 1164 bytes; four seeds make it 460.
    assert_eq!(block(4, 0).len(), 460);
    assert!(block(4, 0).len() * 2 < 16 + HEADER_SIZE + NODE_ADDRESS_LENGTH + 4 * 192);
}

#[test]
fn a_top_up_is_the_same_head_with_no_request() {
    let blocks: Vec<Vec<u8>> = (0..5).map(|_| block(4, 7)).collect();
    let out = repliable_additional_surbs(&[0; 16], &blocks);
    assert_eq!(out[17], TAG_ADDITIONAL_SURBS);
    assert_eq!(TAG_ADDITIONAL_SURBS, 4);
    assert_eq!(u16::from_be_bytes([out[18], out[19]]), 5);
    assert_eq!(out.len(), 22 + 5 * 460);
}
