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

//! Where the mapping cursor finds room.

use crate::linux::abi::errno;
use crate::linux::guest::{page_up, span_within, Guest, MMAP_LIMIT};

/// The first span of `len` at or above the mapping cursor that meets
/// nothing the guest holds.
pub fn free_span(guest: &Guest, len: u64) -> Result<(u64, u64), i64> {
    let mut at = guest.mmap_next;
    loop {
        let (start, span) = span_within(at, len, MMAP_LIMIT).ok_or(errno::ENOMEM)?;
        let end = start + span;
        let past = guest
            .regions
            .iter()
            .filter(|r| r.at < end && start < r.at.saturating_add(r.len))
            .map(|r| r.at.saturating_add(r.len))
            .max();
        match past {
            None => return Ok((start, span)),
            Some(next) => at = page_up(next),
        }
    }
}
