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

//! The request ring, played the way a device plays it: descriptor 0
//! published at the available ring's own position, and each completion
//! read from its own used element.

use super::ring::{Ring, BUF_PHYS, SIZE};
use crate::constants::{ENTROPY_BUF_LEN, VRING_DESC_F_WRITE};

#[test]
fn each_request_publishes_descriptor_zero_at_its_own_ring_position() {
    let ring = Ring::new();
    let mut q = ring.queue();
    // Poison every slot: a slot the driver did not write keeps the poison.
    (0..SIZE as usize).for_each(|p| ring.region.present16(SIZE as usize * 16 + 4 + 2 * p, 0xAAAA));
    for n in 1..=(2 * SIZE) {
        q.post().expect("nothing in flight");
        assert_eq!(ring.avail(None), n);
        assert_eq!(ring.avail(Some(((n - 1) % SIZE) as usize)), 0, "request {n}");
        ring.answer(n as u8);
        assert_eq!(q.completion(), Some(Ok(ENTROPY_BUF_LEN as u32)));
    }
    assert_eq!(ring.region.wrote32(0) as u64 | (ring.region.wrote32(4) as u64) << 32, BUF_PHYS);
    assert_eq!(ring.region.wrote32(8), ENTROPY_BUF_LEN as u32);
    assert_eq!(ring.region.wrote16(12), VRING_DESC_F_WRITE);
}

#[test]
fn each_completion_is_read_from_its_own_used_element() {
    // The second request's length is the second element's, not the first's:
    // a short second fill must not hand out the rest of the buffer as fresh.
    let ring = Ring::new();
    let mut q = ring.queue();
    q.post().expect("posted");
    assert_eq!(q.completion(), None, "nothing completed yet");
    ring.complete(1, 0, 4096, 0x11);
    assert_eq!(q.completion(), Some(Ok(4096)));
    q.post().expect("posted");
    ring.complete(2, 0, 32, 0x22);
    assert_eq!(q.completion(), Some(Ok(32)));
    assert_eq!(q.completion(), None);
}

#[test]
fn completions_wrap_with_the_ring() {
    let ring = Ring::new();
    let mut q = ring.queue();
    for n in 1..=(3 * SIZE as u32) {
        q.post().expect("posted");
        ring.complete(n as u16, 0, n, 0);
        assert_eq!(q.completion(), Some(Ok(n)), "completion {n}");
    }
}
