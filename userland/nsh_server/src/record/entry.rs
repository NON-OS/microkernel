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

//! Appending one entry: `seq: u32 | at_ms: u64 | kind: u8 | body: bytes16`.
//! Each is read back with the checker's parser, chained, and, every
//! `ckpt_every` entries, followed by a checkpoint for the host to sign.

use super::kind::{Kind, Unfit};
use super::{link, Record, CLOSING_ROOM};
use alloc::vec::Vec;
use nsh_wire::record::{FOOTER_LEN, MAX_RECORD_ENTRIES};
use nsh_wire::{Entry, Reader};

impl Record {
    /// Whether an entry of `len` bytes has room.
    pub(crate) fn fits(&self, len: usize, closing: bool) -> bool {
        let keep = if closing { FOOTER_LEN } else { CLOSING_ROOM };
        let entries = if closing { MAX_RECORD_ENTRIES } else { MAX_RECORD_ENTRIES - 3 };
        self.bytes.len() + len + keep <= self.cap && self.count < entries
    }

    /// Append one entry at `now_ms`, or at the last entry's time if the
    /// clock went back. Only the entries that end a session (`closing`)
    /// may use the room kept back for them. A checkpoint's signed fields
    /// come back when one is due.
    pub(crate) fn append(
        &mut self,
        kind: Kind,
        now_ms: u64,
        body: &[u8],
        closing: bool,
    ) -> Result<Option<Vec<u8>>, Unfit> {
        let at_ms = now_ms.max(self.last_ms);
        let n = u16::try_from(body.len()).map_err(|_| Unfit::Body)?;
        let mut e = Vec::with_capacity(15 + body.len());
        e.extend_from_slice(&self.count.to_le_bytes());
        e.extend_from_slice(&at_ms.to_le_bytes());
        e.push(kind as u8);
        e.extend_from_slice(&n.to_le_bytes());
        e.extend_from_slice(body);
        if !self.fits(e.len(), closing) {
            return Err(Unfit::Full);
        }
        let mut r = Reader::new(&e);
        if Entry::read(&mut r).and_then(|_| r.finish()).is_err() {
            return Err(Unfit::Body);
        }
        self.chain = link(&[&self.chain, &e]);
        self.bytes.extend_from_slice(&e);
        self.count += 1;
        self.last_ms = at_ms;
        let due = self.ckpt_every != 0 && self.count.is_multiple_of(self.ckpt_every);
        Ok(due.then(|| self.ckpt_fields()))
    }
}
