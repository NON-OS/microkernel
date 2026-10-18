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

use nonos_virtio::Mmio;

use crate::constants::{TX_BUFFER_LEN, TX_QUEUE_SIZE, VIRTIO_NET_HDR_LEN};

// One bit per buffer in `in_flight`.
const _: () = assert!(TX_QUEUE_SIZE as u32 <= u32::BITS);

/// The transmit virtqueue and its buffers, reached only through their
/// bounds-checked regions.
#[derive(Debug, Clone, Copy)]
pub struct TxQueue {
    pub ring: Mmio,
    pub bufs: Mmio,
    pub buf_phys: u64,
    pub buf_len: u32,
    pub buf_count: u16,
    pub next_avail: u16,
    pub last_used: u16,
    /// Bit `n` set: buffer `n` was posted and the device has not given it
    /// back. Only such a buffer is taken back, whatever id the device names.
    pub in_flight: u32,
    /// When the ring was first found full with the link up, since the last
    /// buffer came back.
    pub full_since: Option<u64>,
    /// Whether the device has given any buffer back since the queue was
    /// built: a stall after that is a new one, not the last one again.
    pub completed: bool,
    /// Bytes of struct virtio_net_hdr written ahead of each frame: the
    /// legacy 10 unless the transport negotiated VERSION_1.
    pub hdr_len: usize,
}

impl TxQueue {
    /// At most TX_QUEUE_SIZE buffers, whatever the count asked for.
    pub fn new(ring: Mmio, bufs: Mmio, buf_phys: u64, buf_count: u16) -> Self {
        Self {
            ring,
            bufs,
            buf_phys,
            buf_len: TX_BUFFER_LEN,
            buf_count: core::cmp::min(buf_count, TX_QUEUE_SIZE),
            next_avail: 0,
            last_used: 0,
            in_flight: 0,
            full_since: None,
            completed: false,
            hdr_len: VIRTIO_NET_HDR_LEN,
        }
    }

    /// The same queue, for a transport whose header is `hdr_len` bytes.
    pub fn with_hdr_len(self, hdr_len: usize) -> Self {
        Self { hdr_len, ..self }
    }

    pub const fn queue_size() -> u16 {
        TX_QUEUE_SIZE
    }
}
