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

//! A slot: which generation, how long its body is, the body's hash, and
//! the objects the body is in, in order. The slot's own hash ends it, so a
//! slot a cut left half written is told from a whole one.
//!
//! ```text
//! 0  magic "NUFS"  4  version u16  6  pad u16  8  generation u64
//! 16 body length u64  24 body BLAKE3 [32]  56 part count u32
//! 60 part ids u64 each; record count u32, then per journal record its
//! object u64, length u32 and BLAKE3 [32], oldest first; then the BLAKE3
//! of every byte before it [32]
//! ```

use alloc::vec::Vec;

use super::{SLOT_MAGIC, VERSION};
use crate::bytes::{In, Out};
use crate::limits::{BODY_MAX, CHUNK};

/// A slot is one object; this many parts fit with room to spare.
pub const SLOT_MAX: usize = CHUNK;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    pub generation: u64,
    pub body_len: u64,
    pub body_hash: [u8; 32],
    pub parts: Vec<u64>,
    /// The journal records applied after the body, oldest first.
    pub journal: Vec<Record>,
}

/// One journal record's object, length and hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u64,
    pub len: u32,
    pub hash: [u8; 32],
}

/// Records before a commit writes the body again: a start reads at most this many.
pub const JOURNAL_MAX: usize = 64;

impl Slot {
    pub fn encode(&self) -> Vec<u8> {
        let mut o = Out(Vec::with_capacity(92 + self.parts.len() * 8));
        o.bytes(&SLOT_MAGIC);
        o.u16(VERSION);
        o.u16(0);
        o.u64(self.generation);
        o.u64(self.body_len);
        o.bytes(&self.body_hash);
        o.u32(self.parts.len() as u32);
        self.parts.iter().for_each(|id| o.u64(*id));
        o.u32(self.journal.len() as u32);
        for r in &self.journal {
            o.u64(r.id);
            o.u32(r.len);
            o.bytes(&r.hash);
        }
        let seal = *blake3::hash(&o.0).as_bytes();
        o.bytes(&seal);
        o.0
    }

    /// None for anything `encode` did not write whole, and for a body
    /// whose parts could not hold it.
    pub fn decode(bytes: &[u8]) -> Option<Slot> {
        let sealed = bytes.len().checked_sub(32)?;
        if blake3::hash(&bytes[..sealed]).as_bytes() != &bytes[sealed..] {
            return None;
        }
        let mut r = In::new(&bytes[..sealed]);
        if r.take(4)? != SLOT_MAGIC || r.u16()? != VERSION || r.u16()? != 0 {
            return None;
        }
        let generation = r.u64()?;
        let body_len = r.u64()?;
        let mut body_hash = [0u8; 32];
        body_hash.copy_from_slice(r.take(32)?);
        let count = r.u32()? as usize;
        if count != (body_len as usize).div_ceil(CHUNK) || body_len > BODY_MAX {
            return None;
        }
        let parts = (0..count).map(|_| r.u64()).collect::<Option<Vec<u64>>>()?;
        let records = r.u32()? as usize;
        if records > JOURNAL_MAX {
            return None;
        }
        let mut journal = Vec::with_capacity(records);
        for _ in 0..records {
            let (id, len) = (r.u64()?, r.u32()?);
            let mut hash = [0u8; 32];
            hash.copy_from_slice(r.take(32)?);
            if len as usize > CHUNK {
                return None;
            }
            journal.push(Record { id, len, hash });
        }
        r.done().then_some(Slot { generation, body_len, body_hash, parts, journal })
    }
}

/// The hash a slot names its body by.
pub fn body_hash(body: &[u8]) -> [u8; 32] {
    *blake3::hash(body).as_bytes()
}
