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

//! A transmit queue's state: its firmware number and its pointers.

use super::super::region::{zero, Region};
use super::layout::{BC, BUFS, TFDS};

pub struct TxQueue {
    pub(super) base: usize,
    /// The firmware's number for the queue, while it has one.
    pub id: u16,
    pub live: bool,
    pub(super) write: u16,
    pub(super) read: u16,
}

impl TxQueue {
    /// The queue whose area starts `base` bytes into the transmit region.
    pub const fn new(base: usize) -> Self {
        Self { base, id: 0, live: false, write: 0, read: 0 }
    }

    /// The descriptor ring's and byte count table's device addresses.
    pub fn tfds<R: Region + ?Sized>(&self, tx: &R) -> u64 {
        tx.dev() + (self.base + TFDS) as u64
    }

    pub fn bc<R: Region + ?Sized>(&self, tx: &R) -> u64 {
        tx.dev() + (self.base + BC) as u64
    }

    /// Clear the queue's memory before the firmware is told of it.
    pub fn clear<R: Region + ?Sized>(&mut self, tx: &R) -> bool {
        self.live = false;
        self.write = 0;
        self.read = 0;
        zero(tx, self.base, BUFS)
    }

    /// Take the firmware's number for the queue and the write pointer its
    /// ring starts at. Until then (and after `clear`) nothing is sent.
    pub fn start(&mut self, id: u16, write_ptr: u16) {
        self.id = id;
        self.live = true;
        self.write = write_ptr;
        self.read = write_ptr;
    }
}
