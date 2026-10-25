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
//! the entries (`record::entry`), then a footer the host keys sign
//! (`record::footer`).

use crate::keys::{HybridPublic, DIGEST_LEN};
use crate::{CapNames, Enrol, SESSION_ID_LEN};

pub const RECORD_MAGIC: [u8; 4] = *b"NSHR";
pub const RECORD_VERSION: u16 = 1;
/// The largest record a checker reads: 64 MiB.
pub const MAX_RECORD: usize = 64 << 20;
pub const MAX_RECORD_ENTRIES: u32 = 1 << 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordHeader<'a> {
    pub session_id: &'a [u8; SESSION_ID_LEN],
    pub start_ms: u64,
    /// `TH(SERVER_HELLO)`, from which, with the host keys, a checker
    /// recomputes the challenge the attestation document must carry.
    pub transcript: &'a [u8; DIGEST_LEN],
    pub host: HybridPublic<'a>,
    pub enrol: Enrol<'a>,
    pub doc_hash: &'a [u8; DIGEST_LEN],
    pub subject: &'a [u8; DIGEST_LEN],
    pub user: &'a [u8],
    pub role: &'a [u8],
    pub grant_hash: &'a [u8; DIGEST_LEN],
    pub caps: CapNames<'a>,
    /// The header's bytes, which start the chain.
    pub bytes: &'a [u8],
}
