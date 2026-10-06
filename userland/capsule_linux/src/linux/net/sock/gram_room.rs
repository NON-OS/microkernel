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

//! What a queued datagram costs its receiver, and whether another one is
//! taken. Pure, so the host proofs hold it.
//!
//! Linux charges each datagram its sk_buff's truesize against SO_RCVBUF:
//! the payload and SKB_TRUESIZE's overhead, an sk_buff and its shared info,
//! 576 bytes on x86_64; and it takes a datagram while what is already
//! charged is not past the buffer (__udp_enqueue_schedule_skb). Counting
//! the payload alone, a datagram of no bytes cost nothing and always fit,
//! so a guest sending empty datagrams to its own socket grew the queue,
//! which is this capsule's memory, without end.

/// SKB_DATA_ALIGN(sizeof(struct sk_buff)) + SKB_DATA_ALIGN(sizeof(struct
/// skb_shared_info)) on x86_64: 256 + 320.
pub const OVERHEAD: usize = 576;

/// What a datagram of `len` bytes is charged while it waits.
pub fn cost(len: usize) -> usize {
    len.saturating_add(OVERHEAD)
}

/// Whether a datagram is taken by a receiver already charged `queued`
/// bytes, with a buffer of `rcvbuf`.
pub fn takes(queued: usize, rcvbuf: u32) -> bool {
    queued <= rcvbuf as usize
}
