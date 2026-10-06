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
//! is the exact address or EEXIST when something is there, whether or not
//! MAP_FIXED is set beside it; any other address is a hint taken only when
//! nothing is there. Otherwise the mapping cursor chooses, skipping every
//! span the guest already holds, since a new mapping laid over an old one
//! would hand back the old pages.

use crate::linux::abi::errno;
use crate::linux::guest::{span_within, Guest, USER_MAX};

use super::map_args::{exact, hint, MAP_FIXED_NOREPLACE};
use super::map_req::MapReq;

pub struct Place {
    pub at: u64,
    pub span: u64,
    /// Chosen by the cursor, which then moves past it.
    pub from_cursor: bool,
}

/// The span the mapping takes, or the errno refusing it.
pub fn place(guest: &Guest, req: &MapReq) -> Result<Place, i64> {
    let exactly = |(at, span)| Place { at, span, from_cursor: false };
    if exact(req.flags) {
        let (at, span) = span_within(req.addr, req.len, USER_MAX).ok_or(errno::ENOMEM)?;
        if req.flags & MAP_FIXED_NOREPLACE != 0 && guest.overlaps(at, span) {
            return Err(errno::EEXIST);
        }
        return Ok(exactly((at, span)));
    }
    if let Some(got) = hint(req.addr).and_then(|at| span_within(at, req.len, USER_MAX)) {
        if !guest.overlaps(got.0, got.1) {
            return Ok(exactly(got));
        }
    }
    let (at, span) = super::map_free::free_span(guest, req.len)?;
    Ok(Place { at, span, from_cursor: true })
}
