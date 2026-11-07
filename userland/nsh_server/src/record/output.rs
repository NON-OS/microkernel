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

//! Terminal output, kept in the record as a count and a SHA-256 per run:
//! the bytes sent since the last entry make one `OUTPUT` entry, written
//! before the next entry of any other kind and before the session ends.

use super::{EntryKind, Record, Unfit};
use alloc::vec::Vec;
use nonos_hash::Sha256;

/// An `OUTPUT` entry: the entry's fields and a 40-byte body.
const OUTPUT_ENTRY: usize = 4 + 8 + 1 + 2 + 40;

#[derive(Default)]
pub(super) struct Pending {
    bytes: u64,
    hash: Option<Sha256>,
}

impl Record {
    /// Count bytes sent as `OUTPUT`.
    pub(crate) fn output(&mut self, bytes: &[u8]) {
        let p = &mut self.output;
        p.hash.get_or_insert_with(Sha256::new).update(bytes);
        p.bytes = p.bytes.saturating_add(bytes.len() as u64);
    }

    /// Write the pending output as one entry, if there is any; `closing`
    /// as `append` takes it.
    pub(crate) fn flush_output(
        &mut self,
        now_ms: u64,
        closing: bool,
    ) -> Result<Option<Vec<u8>>, Unfit> {
        if self.output.hash.is_none() {
            return Ok(None);
        }
        if !self.fits(OUTPUT_ENTRY, closing) {
            return Err(Unfit::Full);
        }
        let Some(hash) = self.output.hash.take() else { return Ok(None) };
        let bytes = core::mem::take(&mut self.output.bytes);
        let mut body = [0u8; 40];
        body[..8].copy_from_slice(&bytes.to_le_bytes());
        body[8..].copy_from_slice(&hash.finalize());
        self.append(EntryKind::Output, now_ms, &body, closing)
    }
}
