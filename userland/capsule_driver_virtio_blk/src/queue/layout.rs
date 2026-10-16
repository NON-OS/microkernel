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

//! The queue's memory and the driver's own count of what it posted and took
//! back. Each grant is reached through a bounds-checked view, so no index
//! touches memory outside it; the ring's copies of the counts are the
//! device's, believed only as they agree with these. Not `Copy`: one owner.

use nonos_virtio::Mmio;

use super::buffer::Buffer;
use super::data::wipe;
use super::fault::Fault;
use super::ring::ring_bytes;
use crate::constants::{DATA_BUF_LEN, STATUS_OFFSET};

pub struct Queue {
    pub(super) ring: Mmio,
    pub(super) header: Buffer,
    pub(super) data: Buffer,
    /// Ring entries the device was told: positions run modulo this.
    pub(super) size: u16,
    /// Requests published, and completions taken.
    pub(super) posted: u16,
    pub(super) used: u16,
    /// The device-writable bytes of the request in flight.
    pub(super) expect: u32,
    /// Why the queue waits for a reset, once it does.
    pub(super) stalled: Option<Fault>,
    /// The device answered OK since the server last asked (`take_served`).
    pub(super) served: bool,
    /// The legacy transport: a used length is believed only as far as the
    /// status byte (`completion`).
    pub(super) legacy: bool,
}

impl Queue {
    /// A queue of `size` entries over `ring`, emptied, so the device starts
    /// with no entry. `None` when a view is too small for what is laid in
    /// it: the ring, the header to the status byte, the most one request moves.
    pub fn new(ring: Mmio, size: u16, header: Buffer, data: Buffer) -> Option<Self> {
        let fits = ring_bytes(size).is_some_and(|n| ring.len() >= n)
            && header.view.len() > STATUS_OFFSET
            && data.view.len() as u64 >= DATA_BUF_LEN;
        if !fits {
            return None;
        }
        wipe(ring, ring.len());
        Some(Self {
            ring,
            header,
            data,
            size,
            posted: 0,
            used: 0,
            expect: 0,
            stalled: None,
            served: false,
            legacy: false,
        })
    }
}
