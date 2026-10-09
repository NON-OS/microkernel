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

use super::error::RecordError;
use super::{P, RECORD_LEN};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Record {
    pub root: [u8; 32],
    pub epoch: u64,
    pub r: [u8; 32],
    pub s: [u8; 32],
}

fn part<const N: usize>(b: &[u8], at: usize) -> [u8; N] {
    let mut out = [0u8; N];
    if let Some(s) = b.get(at..at + N) {
        out.copy_from_slice(s);
    }
    out
}

/// Read a record: its exact length, then every root word canonical.
pub fn parse(b: &[u8]) -> Result<Record, RecordError> {
    if b.len() != RECORD_LEN {
        return Err(RecordError::Length);
    }
    let root: [u8; 32] = part(b, 0);
    if root.chunks_exact(8).any(|w| u64::from_le_bytes(part(w, 0)) >= P) {
        return Err(RecordError::NonCanonicalRoot);
    }
    let epoch = u64::from_le_bytes(part(b, 32));
    Ok(Record { root, epoch, r: part(b, 40), s: part(b, 72) })
}
