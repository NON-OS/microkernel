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

//! Segments that arrive out of order are joined when the gap fills, and a read
//! still hands the reader no more than its reply can carry.

use super::handshake::local_port;
use crate::peer::{connect, drain, fresh, inject, numbers, recv, Seg, ACK};
use crate::protocol::IPC_PAYLOAD_MAX;
use crate::tcp::MSS;

/// Three full segments, `len` bytes each, the first one last.
fn reordered(h: u32, len: usize) -> Vec<u8> {
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    let port = local_port(h);
    let data: Vec<u8> = (0..3 * len).map(|i| (i % 251) as u8).collect();
    for i in [1usize, 2, 0] {
        let seq = rcv_nxt.wrapping_add((i * len) as u32);
        inject(Seg::from_peer(port, seq, snd_nxt, ACK, &data[i * len..(i + 1) * len]));
    }
    data
}

/*
 * Reordering is ordinary on a real path. When the first segment arrives the
 * two held behind it drain as one block of 2920 bytes, and a read copied that
 * block into a reply of 1524: an out of bounds slice, and the capsule aborts.
 */
#[test]
fn a_joined_block_larger_than_a_reply_is_read_in_pieces() {
    let _g = fresh();
    let h = connect();
    let want = reordered(h, 1460);
    let mut got = Vec::new();
    for _ in 0..16 {
        let (errno, part) = recv(h);
        if errno != 0 {
            break;
        }
        assert!(part.len() <= IPC_PAYLOAD_MAX, "one reply carries at most {IPC_PAYLOAD_MAX}");
        got.extend_from_slice(&part);
    }
    assert_eq!(got, want, "every byte, in order, exactly once");
}

#[test]
fn small_reordered_segments_still_arrive_in_order() {
    let _g = fresh();
    let h = connect();
    let want = reordered(h, 100);
    let mut got = Vec::new();
    for _ in 0..8 {
        let (errno, part) = recv(h);
        if errno != 0 {
            break;
        }
        got.extend_from_slice(&part);
    }
    assert_eq!(got, want);
}

/*
 * The advertised window counts the reader's queue in segments, so a block
 * queued whole held up to every reassembled segment under one count: the
 * window said one segment's room was used while thirty two were.
 */
#[test]
fn joined_blocks_are_queued_a_segment_at_a_time() {
    let _g = fresh();
    let h = connect();
    let _ = reordered(h, 1460);
    drain();
    let mut t = crate::state::TABLE.lock();
    let e = t.by_handle_mut(h).expect("connection");
    assert_eq!(e.rx.len(), 3);
    assert!(e.rx.iter().all(|b| b.len() <= MSS), "no queued block is larger than a segment");
    assert_eq!(e.rwnd() as usize, crate::tcp::RWND_MAX as usize - 3 * MSS);
}
