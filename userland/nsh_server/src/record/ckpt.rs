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

//! A checkpoint, `nsh-ckpt/1`: the session, the last entry it covers, the
//! chain value after that entry, and the host keys' hybrid signature over
//! `H("nsh/1 record-ckpt\0" || SHA-512(the fields before it))`. The host
//! keeps checkpoints beside the record, so a record a crash cut short is
//! still checked up to the last one.

use super::Record;
use alloc::vec::Vec;
use nonos_hash::sha512;
use nsh_crypto::{labelled, Label};

pub const CKPT_MAGIC: [u8; 4] = *b"NSHC";
const CKPT_VERSION: u16 = 1;

impl Record {
    /// The signed fields of a checkpoint over the entries so far.
    pub(crate) fn ckpt_fields(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(8 + 16 + 4 + 64);
        out.extend_from_slice(&CKPT_MAGIC);
        out.extend_from_slice(&CKPT_VERSION.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&self.session_id);
        out.extend_from_slice(&(self.count - 1).to_le_bytes());
        out.extend_from_slice(&self.chain);
        out
    }
}

/// The digest the host keys sign for a checkpoint's fields.
pub(crate) fn ckpt_digest(fields: &[u8]) -> [u8; 64] {
    labelled(Label::RecordCkpt, &sha512(fields))
}
