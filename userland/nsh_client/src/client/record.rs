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

//! The record's parts, joined in order up to the one flagged last, held
//! to the 64 MiB cap, and checked to be this session's record.

use super::session::wire;
use super::stage::{Ending, Phase};
use super::{Client, Event};
use crate::ct::eq;
use crate::record::bound::{self, Mismatch};
use crate::Refusal;
use nsh_wire::record::MAX_RECORD;
use nsh_wire::{Reader, RecordHeader, RecordPart};

impl<A, S> Client<A, S> {
    /// One part of the record; the whole record once the last is in.
    pub(super) fn part(&mut self, body: &[u8], p: &mut Phase) -> Result<Option<Event>, Refusal> {
        let part = wire(RecordPart::parse(body))?;
        if self.record.len() + part.bytes.len() > MAX_RECORD {
            return Err(Refusal::RecordLimit);
        }
        self.record.extend_from_slice(part.bytes);
        p.ending = Ending::Record;
        if !part.last {
            return Ok(None);
        }
        p.ending = Ending::RecordDone;
        let record = core::mem::take(&mut self.record);
        self.owns(&record)?;
        Ok(Some(Event::Record(record)))
    }

    /// The record must be this session's: its id, its handshake, this
    /// user, and the document `HOST_PROOF` carried.
    fn owns(&self, record: &[u8]) -> Result<(), Refusal> {
        let h = wire(RecordHeader::read(&mut Reader::new(record)))?;
        bound::check(&h, &self.bound).map_err(Refusal::Record)?;
        match self.doc_hash.is_some_and(|d| eq(&d, h.doc_hash)) {
            true => Ok(()),
            false => Err(Refusal::Record(Mismatch::Document)),
        }
    }
}
