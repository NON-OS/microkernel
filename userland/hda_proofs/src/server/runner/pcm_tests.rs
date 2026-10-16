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

//! PCM from a client, at and past every bound the write takes: its declared
//! length against zero, the chunk limit and the bytes the frame carries, and
//! a queue with no room left for it.

use super::serving::{answer, ask, tx};
use crate::audio::PcmQueue;
use crate::protocol::{E_AGAIN, E_INVAL, E_OK, HDR_LEN, MAX_PCM_CHUNK, OP_WRITE_PCM};
use crate::server::handlers::write_pcm;

/// The status a write declaring `len` bytes, in a frame carrying `body`, gets.
fn write(q: &mut PcmQueue, len: u32, body: usize) -> i32 {
    let frame = vec![0x5a; HDR_LEN + body];
    write_pcm::handle(q, &ask(OP_WRITE_PCM, len), &frame, &mut tx());
    answer(OP_WRITE_PCM).0
}

#[test]
fn a_write_is_taken_only_when_its_length_is_inside_every_bound() {
    let (mut q, max) = (PcmQueue::new(), MAX_PCM_CHUNK);
    assert_eq!(write(&mut q, 0, 16), E_INVAL, "an empty write was taken");
    assert_eq!(write(&mut q, max as u32 + 1, max + 1), E_INVAL, "past the chunk limit");
    assert_eq!(write(&mut q, 100, 99), E_INVAL, "longer than the frame carries");
    assert_eq!(write(&mut q, u32::MAX, max), E_INVAL, "a length no frame can carry");
    assert_eq!(write(&mut q, max as u32, max), E_OK, "a whole chunk was refused");
    assert_eq!(write(&mut q, 1, 1), E_OK);
}

#[test]
fn a_full_queue_asks_the_writer_to_wait_rather_than_dropping_or_overrunning() {
    let mut q = PcmQueue::new();
    for _ in 0..16 {
        assert_eq!(write(&mut q, MAX_PCM_CHUNK as u32, MAX_PCM_CHUNK), E_OK);
    }
    assert_eq!(write(&mut q, 1, 1), E_AGAIN, "64 KiB queued and still taking more");
    let mut out = vec![0u8; MAX_PCM_CHUNK];
    assert_eq!(q.pop_into(&mut out), MAX_PCM_CHUNK);
    assert!(out.iter().all(|&b| b == 0x5a), "the queue gave back other bytes");
    assert_eq!(write(&mut q, MAX_PCM_CHUNK as u32, MAX_PCM_CHUNK), E_OK, "room made, refused");
}
