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

//! Walking an mprotect span one mapping at a time.

use crate::linux::abi::errno;
use crate::linux::guest::{maps_full, Guest};

use super::prot::{PROT_ANY, PROT_EXEC, PROT_WRITE};
use super::prot_span::protect_span;

/// Give `[start, start + span)` the protection `prot`, mapping by mapping.
pub(super) fn walk(guest: &mut Guest, start: u64, span: u64, prot: u64) -> u64 {
    let end = start + span;
    let mut at = start;
    while at < end {
        let Some(r) = guest.regions.iter().find(|r| r.at <= at && at < r.at + r.len).copied()
        else {
            /* Linux refuses a span with no mapping in it at all. */
            return errno::fail(errno::ENOMEM);
        };
        let upto = end.min(r.at + r.len);
        let piece = upto - at;
        if !r.backed {
            /*
             * A PROT_NONE reservation has no pages for the kernel to
             * reprotect. Asking for access commits it, which is how musl makes
             * a thread stack: reserve with PROT_NONE, then mprotect the part
             * it uses to read-write. PROT_NONE on it changes nothing. The
             * commit maps the piece with `prot` and records it.
             */
            if prot & PROT_ANY != 0
                && guest.commit(at, piece, prot & PROT_WRITE != 0, prot & PROT_EXEC != 0) < 0
            {
                return errno::fail(errno::ENOMEM);
            }
        } else if maps_full(guest.regions.len(), guest.regions.len() + 2) {
            /*
             * A change to the middle of a mapping splits it in three, and
             * Linux refuses one that would pass vm.max_map_count.
             */
            return errno::fail(errno::ENOMEM);
        } else if protect_span(guest, at, piece, prot) < 0 {
            return errno::fail(errno::EACCES);
        }
        at = upto;
    }
    errno::ok(0)
}
