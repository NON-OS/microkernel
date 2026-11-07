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

//! Starting a record: the header written, read back with the checker's own
//! parser, and the chain started from it.

use super::header::{Header, HEADER_MAX};
use super::{link, output::Pending, Record};
use alloc::vec;
use alloc::vec::Vec;
use nsh_wire::record::{RECORD_MAGIC, RECORD_VERSION};
use nsh_wire::{CapNames, Reader, RecordHeader, WireError, Writer};

impl Record {
    /// A record for this session, read back with the checker's own parser
    /// before anything is chained over it.
    pub(crate) fn start(h: &Header<'_>, cap: usize, ckpt_every: u32) -> Result<Record, WireError> {
        let mut buf = vec![0u8; HEADER_MAX];
        let mut w = Writer::new(&mut buf);
        w.put(&RECORD_MAGIC)?;
        w.u16(RECORD_VERSION)?;
        w.u16(0)?;
        w.put(&h.session_id)?;
        w.u64(h.start_ms)?;
        w.put(h.th_server_hello)?;
        w.put(h.host_ed)?;
        w.put(h.host_ml)?;
        w.bytes16(h.enrol)?;
        w.put(h.doc_hash)?;
        w.put(h.subject)?;
        w.bytes8(h.user)?;
        w.bytes8(h.role)?;
        w.put(h.grant_hash)?;
        CapNames::write(&mut w, h.caps)?;
        let n = w.done();
        buf.truncate(n);
        let mut r = Reader::new(&buf);
        RecordHeader::read(&mut r)?;
        r.finish()?;
        let bytes: Vec<u8> = buf;
        let chain = link(&[b"nsh/1 record\0", &bytes]);
        Ok(Record {
            session_id: h.session_id,
            chain,
            count: 0,
            last_ms: h.start_ms,
            cap,
            ckpt_every,
            output: Pending::default(),
            handed: 0,
            bytes,
        })
    }
}
