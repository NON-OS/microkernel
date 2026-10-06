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

//! The request ring, against a region in host memory played the way a
//! device plays it: descriptor 0 published at the available ring's own
//! position, and each completion read from its own used element, with an
//! element naming another descriptor dropped.

use crate::constants::{ENTROPY_BUF_LEN, VQ_REGION_SIZE, VQ_USED_OFFSET, VRING_DESC_F_WRITE};
use crate::queue::Queue;

/// QEMU's virtio-rng offers an 8-entry queue.
const SIZE: u16 = 8;
const BUF_PHYS: u64 = 0x7700_0000;

#[repr(align(4096))]
struct Region([u8; VQ_REGION_SIZE]);

struct Ring {
    region: Box<Region>,
    buf: Box<[u8; ENTROPY_BUF_LEN as usize]>,
}

impl Ring {
    fn new() -> Self {
        Self { region: Box::new(Region([0; VQ_REGION_SIZE])), buf: Box::new([0; 4096]) }
    }

    fn queue(&mut self) -> Queue {
        Queue::new(
            self.region.0.as_mut_ptr() as u64,
            0,
            self.buf.as_mut_ptr() as u64,
            BUF_PHYS,
            ENTROPY_BUF_LEN as u32,
            SIZE,
        )
    }

    fn u16_at(&self, off: usize) -> u16 {
        u16::from_le_bytes([self.region.0[off], self.region.0[off + 1]])
    }

    fn avail_idx(&self) -> u16 {
        self.u16_at(SIZE as usize * 16 + 2)
    }

    fn avail_entry(&self, pos: usize) -> u16 {
        self.u16_at(SIZE as usize * 16 + 4 + 2 * pos)
    }

    /// The device's side: one used element, then the index that covers it.
    fn complete(&mut self, used_idx: u16, id: u32, len: u32) {
        let pos = (used_idx.wrapping_sub(1) % SIZE) as usize;
        let elem = VQ_USED_OFFSET + 4 + 8 * pos;
        self.region.0[elem..elem + 4].copy_from_slice(&id.to_le_bytes());
        self.region.0[elem + 4..elem + 8].copy_from_slice(&len.to_le_bytes());
        self.region.0[VQ_USED_OFFSET + 2..VQ_USED_OFFSET + 4].copy_from_slice(&used_idx.to_le_bytes());
    }
}

#[test]
fn each_request_publishes_descriptor_zero_at_its_own_ring_position() {
    let mut ring = Ring::new();
    let q = ring.queue();
    // Poison every slot: a slot the driver did not write keeps the poison.
    for pos in 0..SIZE as usize {
        let at = SIZE as usize * 16 + 4 + 2 * pos;
        ring.region.0[at..at + 2].copy_from_slice(&0xAAAAu16.to_le_bytes());
    }
    for n in 1..=(2 * SIZE) {
        q.post_request();
        assert_eq!(ring.avail_idx(), n);
        assert_eq!(ring.avail_entry(((n - 1) % SIZE) as usize), 0, "request {n}");
    }
    let desc = &ring.region.0[..16];
    assert_eq!(u64::from_le_bytes(desc[0..8].try_into().unwrap()), BUF_PHYS);
    assert_eq!(u32::from_le_bytes(desc[8..12].try_into().unwrap()), ENTROPY_BUF_LEN as u32);
    assert_eq!(u16::from_le_bytes([desc[12], desc[13]]), VRING_DESC_F_WRITE);
}

#[test]
fn each_completion_is_read_from_its_own_used_element() {
    // The second request's length is the second element's, not the first's:
    // a short second fill must not hand out the rest of the buffer as fresh.
    let mut ring = Ring::new();
    let mut q = ring.queue();
    assert_eq!(q.completion(), None, "nothing completed yet");
    ring.complete(1, 0, 4096);
    assert_eq!(q.completion(), Some(Ok(4096)));
    ring.complete(2, 0, 32);
    assert_eq!(q.completion(), Some(Ok(32)));
    assert_eq!(q.completion(), None);
}

#[test]
fn completions_wrap_with_the_ring() {
    let mut ring = Ring::new();
    let mut q = ring.queue();
    for n in 1..=(3 * SIZE as u32) {
        ring.complete(n as u16, 0, n);
        assert_eq!(q.completion(), Some(Ok(n)), "completion {n}");
    }
}

#[test]
fn an_element_naming_another_descriptor_is_dropped() {
    let mut ring = Ring::new();
    let mut q = ring.queue();
    for id in [1u32, 7, 8, u32::MAX] {
        let next = q.last_used.wrapping_add(1);
        ring.complete(next, id, 64);
        assert!(q.completion().expect("an element").is_err(), "id {id}");
        assert_eq!(q.last_used, next, "the element is consumed, not retried");
    }
}

#[test]
fn a_length_past_the_buffer_is_held_to_the_buffer() {
    let mut ring = Ring::new();
    let mut q = ring.queue();
    for (n, len) in [4097u32, 0x1_0000, u32::MAX].into_iter().enumerate() {
        ring.complete(n as u16 + 1, 0, len);
        assert_eq!(q.completion(), Some(Ok(ENTROPY_BUF_LEN as u32)));
    }
}
