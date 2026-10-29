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

//! One connection: the pid that opened it, the door end it moves bytes
//! through, the one READ it may hold, and its last READ and WRITE answers
//! with their numbers. Asked again under the same number, a question gets
//! the kept answer, so nothing is taken twice or lost when the asker's call
//! ran out (nonos_serve_wire bodies.rs).

use alloc::vec::Vec;

use nonos_serve_wire::{CLOSED, OK};

use crate::linux::abi::errno::EAGAIN;
use crate::linux::net::sock::Socks;

pub struct Conn {
    pub id: u16,
    /// Only its frames count.
    pub pid: u32,
    /// The door end, which no guest holds (sock/door.rs).
    pub end: u32,
    pub held: Option<Held>,
    /// The last READ answered: its number, status and body.
    pub read: Option<(u32, u8, Vec<u8>)>,
    /// The last WRITE answered: its number and how many bytes were taken.
    pub wrote: Option<(u32, u32)>,
}

/// A READ waiting for the guest to write.
#[derive(Clone, Copy)]
pub struct Held {
    /// The request its answer goes to.
    pub request: u64,
    pub seq: u32,
    pub want: u32,
    /// When it is answered empty, in mk_uptime_ms milliseconds.
    pub until: u64,
}

impl Conn {
    pub fn new(id: u16, pid: u32, end: u32) -> Self {
        Self { id, pid, end, held: None, read: None, wrote: None }
    }

    /// READ `seq`'s answer, kept to give again when it is asked again.
    pub(super) fn keep(&mut self, seq: u32, said: (u8, Vec<u8>)) -> (u8, Vec<u8>) {
        self.read = Some((seq, said.0, said.1.clone()));
        said
    }
}

/// What the door end holds for a READ: bytes, CLOSED at the guest's end of
/// file or reset, or None while there is nothing yet.
pub(super) fn take(t: &mut Socks, end: u32, want: u32) -> Option<(u8, Vec<u8>)> {
    match t.read(end, want as usize, false) {
        Ok(bytes) if !bytes.is_empty() => Some((OK, bytes)),
        Err(EAGAIN) => None,
        _ => Some((CLOSED, Vec::new())),
    }
}
