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

//! Reading a whole record. Entries are read until only a footer's worth of
//! bytes is left; their sequence numbers must count up from 0 without a
//! gap, their times may not go back, and the footer's count and end time
//! must agree with them. The chain and the signature are the checker's.

use crate::keys::before_sig;
use crate::record::entry::Entry;
use crate::record::footer::{Footer, FOOTER_LEN};
use crate::record::header::{RecordHeader, MAX_RECORD, MAX_RECORD_ENTRIES};
use crate::{Reader, WireError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Record<'a> {
    pub header: RecordHeader<'a>,
    pub footer: Footer<'a>,
    /// The bytes the host keys signed: all but the signature.
    pub signed: &'a [u8],
    entries: &'a [u8],
}

impl<'a> Record<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, WireError> {
        if bytes.len() > MAX_RECORD {
            return Err(WireError::Oversize);
        }
        let mut r = Reader::new(bytes);
        let header = RecordHeader::read(&mut r)?;
        let start = r.pos;
        let (mut count, mut last_ms) = (0u32, header.start_ms);
        while r.left() > FOOTER_LEN {
            let e = Entry::read(&mut r)?;
            if e.seq != count || e.at_ms < last_ms {
                return Err(WireError::Value);
            }
            if count == MAX_RECORD_ENTRIES {
                return Err(WireError::Oversize);
            }
            (count, last_ms) = (count + 1, e.at_ms);
        }
        let entries = &bytes[start..r.pos];
        let footer = Footer::read(&mut r)?;
        r.finish()?;
        if footer.count != count || footer.end_ms < last_ms {
            return Err(WireError::Value);
        }
        Ok(Record { header, footer, signed: before_sig(bytes)?, entries })
    }

    /// The entries in order, each already checked by `parse`.
    pub fn entries(&self) -> impl Iterator<Item = Entry<'a>> + 'a {
        let mut r = Reader::new(self.entries);
        core::iter::from_fn(move || match r.left() {
            0 => None,
            _ => Entry::read(&mut r).ok(),
        })
    }
}
