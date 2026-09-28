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

//! Where an mmap goes. MAP_FIXED is the exact address; MAP_FIXED_NOREPLACE
//! is the exact address or EEXIST when something is there; any other address
//! is a hint taken only when nothing is there. Otherwise the mapping cursor
//! chooses, skipping every span the guest already holds, since a new mapping
//! laid over an old one would hand back the old pages.

use crate::linux::abi::errno;
use crate::linux::guest::{page_up, span_within, Guest, MMAP_LIMIT, USER_MAX};

use super::map_req::MapReq;

const MAP_FIXED: u64 = 0x10;
const MAP_FIXED_NOREPLACE: u64 = 0x10_0000;

pub struct Place {
    pub at: u64,
    pub span: u64,
    /// Chosen by the cursor, which then moves past it.
    pub from_cursor: bool,
}

/// The span the mapping takes, or the errno refusing it.
pub fn place(guest: &Guest, req: &MapReq) -> Result<Place, i64> {
    let exact = |(at, span)| Place { at, span, from_cursor: false };
    if req.flags & (MAP_FIXED | MAP_FIXED_NOREPLACE) != 0 {
        let (at, span) = span_within(req.addr, req.len, USER_MAX).ok_or(errno::ENOMEM)?;
        if req.flags & MAP_FIXED == 0 && guest.overlaps(at, span) {
            return Err(errno::EEXIST);
        }
        return Ok(exact((at, span)));
    }
    // Linux rounds a hint up to a page.
    if req.addr != 0 {
        if let Some(got) = span_within(page_up(req.addr), req.len, USER_MAX) {
            if !guest.overlaps(got.0, got.1) {
                return Ok(exact(got));
            }
        }
    }
    let mut at = guest.mmap_next;
    loop {
        let (start, span) = span_within(at, req.len, MMAP_LIMIT).ok_or(errno::ENOMEM)?;
        let end = start + span;
        let past = guest
            .regions
            .iter()
            .filter(|r| r.at < end && start < r.at.saturating_add(r.len))
            .map(|r| r.at.saturating_add(r.len))
            .max();
        match past {
            None => return Ok(Place { at: start, span, from_cursor: true }),
            Some(next) => at = page_up(next),
        }
    }
}
