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

//! Giving a guest's pages back.

use nonos_libc::peer::mk_peer_unmap;

use crate::linux::abi::errno;

use super::handle::Guest;
use super::layout::USER_MAX;
use super::mem::{maps_full, span_within, MAX_SPAN};
use super::region_cut::cut;

impl Guest {
    /// Return `[addr, addr + len)` to the kernel. `-ENOMEM` when cutting it
    /// out would leave more spans than vm.max_map_count, as Linux refuses it.
    pub fn unmap(&mut self, addr: u64, len: u64) -> i64 {
        let Some((start, span)) = span_within(addr, len, USER_MAX) else {
            return -1;
        };
        /*
         * A hole in the middle of a mapping leaves two. Unrefused, a guest
         * could split a reservation, which costs it no frame, into as many
         * entries of this capsule's memory as it liked.
         */
        let kept = cut(&self.regions, start, span);
        if maps_full(self.regions.len(), kept.len()) {
            return -errno::ENOMEM;
        }
        let rc = self.drop_frames(start, span);
        if rc < 0 {
            return rc;
        }
        self.regions = kept;
        0
    }

    /// Take the frames under `[start, start + span)`, a page-aligned span,
    /// back from the guest and leave its region list alone: a page not there
    /// is skipped, and one touched again reads zero.
    pub fn drop_frames(&self, start: u64, span: u64) -> i64 {
        let mut done = 0;
        while done < span {
            let take = (span - done).min(MAX_SPAN);
            let rc = mk_peer_unmap(self.pid, start + done, take);
            if rc < 0 {
                return rc;
            }
            done += take;
        }
        0
    }
}
