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

//! Reading a record's header. Every field is placed and shaped here; that
//! the header agrees with the attestation document and the grant it names
//! is the checker's work.

use crate::host_proof::MAX_ENROL;
use crate::keys::HybridPublic;
use crate::names::{self, MAX_ROLE, MAX_USER};
use crate::record::header::{RecordHeader, RECORD_MAGIC, RECORD_VERSION};
use crate::{CapNames, Enrol, Reader, WireError};

impl<'a> RecordHeader<'a> {
    pub fn read(r: &mut Reader<'a>) -> Result<Self, WireError> {
        let start = r.pos;
        r.preamble(&RECORD_MAGIC, RECORD_VERSION)?;
        let session_id = r.array()?;
        let start_ms = r.u64()?;
        let transcript = r.array()?;
        let host = HybridPublic::read(r)?;
        let enrol = Enrol::parse(r.bytes16(1, MAX_ENROL)?)?;
        let (doc_hash, subject) = (r.array()?, r.array()?);
        let user = r.bytes8(1, MAX_USER)?;
        names::user(user)?;
        let role = r.bytes8(1, MAX_ROLE)?;
        names::role(role)?;
        let grant_hash = r.array()?;
        let caps = CapNames::read(r)?;
        if caps.is_empty() {
            return Err(WireError::Value);
        }
        let bytes = &r.buf[start..r.pos];
        Ok(RecordHeader {
            session_id,
            start_ms,
            transcript,
            host,
            enrol,
            doc_hash,
            subject,
            user,
            role,
            grant_hash,
            caps,
            bytes,
        })
    }
}
