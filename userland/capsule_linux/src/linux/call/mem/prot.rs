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

//! `mprotect`, and the rule that makes it necessary.

use crate::linux::abi::errno;
use crate::linux::guest::{span_within, Guest, USER_MAX};

use super::prot_span::protect_span;

pub const PROT_WRITE: u64 = 2;
pub const PROT_EXEC: u64 = 4;
/// PROT_READ, PROT_WRITE and PROT_EXEC together: any access at all.
pub const PROT_ANY: u64 = 7;

/// A request for both at once.
pub fn wx_refused(prot: u64) -> bool {
    prot & PROT_WRITE != 0 && prot & PROT_EXEC != 0
}

pub fn mprotect(guest: &mut Guest, addr: u64, len: u64, prot: u64) -> u64 {
    if len == 0 {
        return errno::ok(0);
    }
    if wx_refused(prot) {
        return errno::fail(errno::EPERM);
    }
    /*
     * Checked, because `len` is the guest's: `addr + len` wraps and the
     * span computed from the wrapped value comes out enormous.
     */
    let Some((start, span)) = span_within(addr, len, USER_MAX) else {
        return errno::fail(errno::EINVAL);
    };
    /*
     * A file mapped without exec was never proved, and making it executable
     * now would run bytes the exec path would have refused. Anonymous memory
     * may still become executable, as a JIT needs; that is the guest's own
     * code, confined by its token rather than by provenance.
     */
    if prot & PROT_EXEC != 0 && guest.span_unproven(start, span) {
        return errno::fail(errno::EPERM);
    }
    let end = start + span;
    let mut at = start;
    while at < end {
        let Some(r) = guest.regions.iter().find(|r| r.at <= at && at < r.at + r.len).copied()
        else {
            // Linux refuses a span with no mapping in it at all.
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
        } else if protect_span(guest, at, piece, prot) < 0 {
            return errno::fail(errno::EACCES);
        }
        at = upto;
    }
    errno::ok(0)
}
