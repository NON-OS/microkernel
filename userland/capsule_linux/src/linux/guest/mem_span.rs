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

//! Backing a span of a guest, a megabyte at a time.

use nonos_libc::peer::{mk_peer_map, mk_peer_unmap};

use super::mem::MAX_SPAN;

/// `MkPeerMap` over a span, a megabyte at a time.
pub(super) fn map_span(pid: u32, at: u64, len: u64, prot: u64) -> i64 {
    let mut done = 0;
    while done < len {
        let take = (len - done).min(MAX_SPAN);
        let rc = mk_peer_map(pid, at + done, take, prot);
        if rc < 0 {
            /*
             * The kernel maps page by page and stops at the first it cannot
             * back. What it did map is given back: left, those frames stay
             * pinned in a span the guest was told it does not have.
             */
            unmap_span(pid, at, done + take);
            return rc;
        }
        done += take;
    }
    0
}

fn unmap_span(pid: u32, at: u64, len: u64) {
    let mut done = 0;
    while done < len {
        let take = (len - done).min(MAX_SPAN);
        let _ = mk_peer_unmap(pid, at + done, take);
        done += take;
    }
}
