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

//! The session record, `nsh-record/1` (docs/design/nsh-v1-record.md): who
//! logged in under which attestation and grant, what they were given, then
//! the entries (`record_entry`), then a footer the host keys sign
//! (`record_footer`).

use crate::keys::DIGEST_LEN;
use crate::names::{self, MAX_ROLE, MAX_USER};
use crate::{CapNames, Reader, WireError, SESSION_ID_LEN};

pub const RECORD_MAGIC: [u8; 4] = *b"NSHR";
pub const RECORD_VERSION: u16 = 1;
/// The largest record a checker reads: 64 MiB.
pub const MAX_RECORD: usize = 64 << 20;
pub const MAX_RECORD_ENTRIES: u32 = 1 << 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordHeader<'a> {
    pub session_id: &'a [u8; SESSION_ID_LEN],
    pub start_ms: u64,
    pub doc_hash: &'a [u8; DIGEST_LEN],
    pub host_keys: &'a [u8; DIGEST_LEN],
    pub subject: &'a [u8; DIGEST_LEN],
    pub user: &'a [u8],
    pub role: &'a [u8],
    pub grant_hash: &'a [u8; DIGEST_LEN],
    pub caps: CapNames<'a>,
    /// The header's bytes, which start the chain.
    pub bytes: &'a [u8],
}

impl<'a> RecordHeader<'a> {
    pub fn read(r: &mut Reader<'a>) -> Result<Self, WireError> {
        let start = r.pos;
        r.preamble(&RECORD_MAGIC, RECORD_VERSION)?;
        let session_id = r.array()?;
        let start_ms = r.u64()?;
        let (doc_hash, host_keys, subject) = (r.array()?, r.array()?, r.array()?);
        let user = r.bytes8(1, MAX_USER)?;
        names::user(user)?;
        let role = r.bytes8(1, MAX_ROLE)?;
        names::role(role)?;
        let grant_hash = r.array()?;
        let caps = CapNames::read(r)?;
        let bytes = &r.buf[start..r.pos];
        Ok(RecordHeader {
            session_id,
            start_ms,
            doc_hash,
            host_keys,
            subject,
            user,
            role,
            grant_hash,
            caps,
            bytes,
        })
    }
}
