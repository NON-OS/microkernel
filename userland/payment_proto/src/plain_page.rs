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

//! ORDER_PLAINTEXT's reply: total u64 LE | offset u64 LE | up to
//! PLAINTEXT_PAGE bytes from offset.

use alloc::vec::Vec;

use crate::cursor::Cursor;

pub const PLAINTEXT_PAGE: usize = 60 * 1024;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlainPage {
    pub total: u64,
    pub offset: u64,
    pub bytes: Vec<u8>,
}

impl PlainPage {
    /// The page of `plain` at `offset`; None past its end.
    pub fn of(plain: &[u8], offset: u64) -> Option<Self> {
        let start = usize::try_from(offset).ok()?;
        let rest = plain.get(start..).filter(|r| !r.is_empty() || plain.is_empty())?;
        let bytes = rest.get(..rest.len().min(PLAINTEXT_PAGE))?.to_vec();
        Some(PlainPage { total: plain.len() as u64, offset, bytes })
    }

    pub fn encode(&self) -> Vec<u8> {
        [&self.total.to_le_bytes()[..], &self.offset.to_le_bytes(), &self.bytes].concat()
    }

    /// None for a page longer than PLAINTEXT_PAGE or past its total.
    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (total, offset) = (r.u64()?, r.u64()?);
        let bytes = b.get(16..)?.to_vec();
        let end = offset.checked_add(bytes.len() as u64)?;
        (bytes.len() <= PLAINTEXT_PAGE && end <= total).then_some(PlainPage {
            total,
            offset,
            bytes,
        })
    }
}
