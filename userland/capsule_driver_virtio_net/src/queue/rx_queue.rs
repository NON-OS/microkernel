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

use crate::constants::{RX_BUFFER_LEN, RX_QUEUE_SIZE, VIRTIO_NET_HDR_LEN};

/// The receive virtqueue and its buffers. Both are reached only through
/// their bounds-checked regions; the device writes them at any time.
#[derive(Debug, Clone, Copy)]
pub struct RxQueue {
    /// The ring: descriptors, avail ring, used ring.
    pub ring: Mmio,
    /// `buf_count` buffers of `buf_len` bytes, back to back.
    pub bufs: Mmio,
    /// The buffers' device address.
    pub buf_phys: u64,
    pub buf_len: u32,
    pub buf_count: u16,
    /// Used entries taken so far.
    pub last_used: u16,
    /// Avail entries offered so far.
    pub next_avail: u16,
    /// Bytes of struct virtio_net_hdr ahead of each received frame: the
    /// legacy 10 unless the transport negotiated VERSION_1.
    pub hdr_len: usize,
}

impl RxQueue {
    /// At most RX_QUEUE_SIZE buffers, whatever the count asked for.
    pub fn new(ring: Mmio, bufs: Mmio, buf_phys: u64, buf_count: u16) -> Self {
        Self {
            ring,
            bufs,
            buf_phys,
            buf_len: RX_BUFFER_LEN,
            buf_count: core::cmp::min(buf_count, RX_QUEUE_SIZE),
            last_used: 0,
            next_avail: 0,
            hdr_len: VIRTIO_NET_HDR_LEN,
        }
    }

    /// The same queue, for a transport whose header is `hdr_len` bytes.
    pub fn with_hdr_len(self, hdr_len: usize) -> Self {
        Self { hdr_len, ..self }
    }

    pub const fn queue_size() -> u16 {
        RX_QUEUE_SIZE
    }
}
