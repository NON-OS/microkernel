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

//! The bond store's `Keys` over the keyring. Requests that carry a record,
//! and answers that do, are wiped once read; the caller wipes what
//! `retrieve` hands it, as `Keys` says.

use alloc::vec::Vec;

use super::answer::{id, ids, refusal};
use super::wire::{self, OP_DELETE, OP_LIST, OP_RETRIEVE, OP_STORE};
use nonos_bt_bond::{wipe, Keys, KeysError, MEMORY_RECORDS};

/// The longest record kept, a bond of 48 octets, with room to spare.
const RECORD_ROOM: usize = 64;
/// A count and the ids of one owner's sixteen records.
const LIST_ROOM: usize = 4 + 4 * MEMORY_RECORDS;

pub trait Call {
    /// One request; the answer's payload, at most `room` octets, or the
    /// keyring's errno (-11 when it did not answer).
    fn call(&mut self, op: u16, payload: &[u8], room: usize) -> Result<Vec<u8>, i32>;
    fn now_ms(&mut self) -> u64;
}

pub struct Keyring<C> {
    call: C,
    /// This capsule's own pid, from getpid: the one the kernel stamps.
    pid: u32,
}

impl<C: Call> Keyring<C> {
    pub fn new(call: C, pid: u32) -> Self {
        Self { call, pid }
    }
}

impl<C: Call> Keys for Keyring<C> {
    fn store(&mut self, kind: u8, data: &[u8]) -> Result<u32, KeysError> {
        let now = self.call.now_ms();
        let mut p = wire::store(self.pid, now, kind, data).ok_or(KeysError::Unavailable)?;
        let answer = self.call.call(OP_STORE, &p, 4);
        wipe(&mut p);
        id(&answer.map_err(refusal)?)
    }

    fn retrieve(&mut self, id: u32) -> Result<Vec<u8>, KeysError> {
        let p = wire::by_id(self.pid, id);
        self.call.call(OP_RETRIEVE, &p, RECORD_ROOM).map_err(refusal)
    }

    fn delete(&mut self, id: u32) -> Result<(), KeysError> {
        let p = wire::by_id(self.pid, id);
        self.call.call(OP_DELETE, &p, 0).map(drop).map_err(refusal)
    }

    fn list(&mut self, kind: u8) -> Result<Vec<u32>, KeysError> {
        let p = wire::list(self.pid, kind);
        ids(&self.call.call(OP_LIST, &p, LIST_ROOM).map_err(refusal)?)
    }
}
