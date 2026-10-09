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

//! The records a batched read hands over.
//!
//! A batched read answers with as many delivered messages as fit, each as a
//! record: a flag byte, the length as a little endian u32, then the bytes.
//! A message too long for one answer goes as several records, the first
//! flagged that more follows and each later one flagged as continuing, so a
//! reader can put it back together and never mistakes a piece for a whole.

/// Flag byte and length.
pub const RECORD_HEADER: usize = 5;

/// These bytes continue a message an earlier record began.
pub const FLAG_CONT: u8 = 1;

/// The message goes on in a later record.
pub const FLAG_MORE: u8 = 2;

/// Write one record into `out` at `at`, returning where the next one starts,
/// or `None` if it does not fit.
pub fn put_record(out: &mut [u8], at: usize, flags: u8, bytes: &[u8]) -> Option<usize> {
    let len = u32::try_from(bytes.len()).ok()?;
    let body = at.checked_add(RECORD_HEADER)?;
    let end = body.checked_add(bytes.len())?;
    let slot = out.get_mut(at..end)?;
    slot[0] = flags;
    slot[1..RECORD_HEADER].copy_from_slice(&len.to_le_bytes());
    slot[RECORD_HEADER..].copy_from_slice(bytes);
    Some(end)
}
