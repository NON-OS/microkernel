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

//! Copying a parent's spans into the child it just made.

use crate::linux::guest::{Guest, MAX_SPAN};
use nonos_libc::peer::{mk_peer_map, mk_peer_write};

/// Every span, mapped into the child and then filled from the parent.
pub(super) fn copy_spans(guest: &mut Guest, child: u32) -> bool {
    let spans = guest.regions.clone();
    for span in spans {
        /*
         * An unbacked reservation has no frames to copy; the child holds the
         * same reservation, and a touch there faults in the child as here.
         */
        if !span.backed {
            continue;
        }
        /*
         * The kernel maps and copies at most MAX_SPAN in one peer call, so a
         * larger span, which a Go heap is, crosses in pieces. Each piece gets
         * the protection the span has now, PROT_NONE included: the kernel
         * copies into a page whatever its protection, so the bytes still go
         * in, and the child can do no more with them than the parent can.
         */
        let mut done = 0;
        while done < span.len {
            let take = (span.len - done).min(MAX_SPAN);
            if mk_peer_map(child, span.at + done, take, span.peer_prot()) < 0 {
                return false;
            }
            if !copy_one(guest, child, span.at + done, take) {
                return false;
            }
            done += take;
        }
    }
    true
}

fn copy_one(guest: &Guest, child: u32, at: u64, len: u64) -> bool {
    let Some(bytes) = guest.read(at, len as usize) else {
        return false;
    };
    mk_peer_write(child, at, &bytes) >= 0
}
