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

//! One record entry: `seq: u32 | at_ms: u64 | kind: u8 | body: bytes16`.
//! The kinds and their bodies are in `record::body`.

use crate::keys::DIGEST_LEN;
use crate::record::boxed::BoxRun;
use crate::{CapNames, Reader, Reason, WireError};

/// The longest command line a record keeps.
pub const MAX_COMMAND: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryBody<'a> {
    Command(&'a [u8]),
    Output {
        bytes: u64,
        digest: &'a [u8; 32],
    },
    RefusedCall {
        tag: u32,
        cap: &'a [u8],
    },
    EscGranted {
        grant: &'a [u8; DIGEST_LEN],
        expires_ms: u64,
        caps: CapNames<'a>,
    },
    EscRefused {
        reason: Reason,
        grant: &'a [u8; DIGEST_LEN],
    },
    EscEnded {
        grant: &'a [u8; DIGEST_LEN],
        why: u8,
    },
    Exit(i32),
    /// SHA-256 of a command line, for a role that keeps no command text.
    CommandDigest(&'a [u8; 32]),
    Box(BoxRun<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry<'a> {
    pub seq: u32,
    pub at_ms: u64,
    pub body: EntryBody<'a>,
    /// The entry's bytes, which the chain folds in.
    pub bytes: &'a [u8],
}

impl<'a> Entry<'a> {
    pub fn read(r: &mut Reader<'a>) -> Result<Self, WireError> {
        let start = r.pos;
        let seq = r.u32()?;
        let at_ms = r.u64()?;
        let kind = r.u8()?;
        let body = EntryBody::parse(kind, r.bytes16(1, MAX_COMMAND)?)?;
        Ok(Entry { seq, at_ms, body, bytes: &r.buf[start..r.pos] })
    }
}
