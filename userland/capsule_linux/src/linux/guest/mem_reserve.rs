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

//! Address space taken ahead of use, and committing part of it.

use nonos_libc::peer::{mk_peer_map, PEER_PROT_EXEC, PEER_PROT_WRITE};

use super::handle::Guest;
use super::layout::USER_MAX;
use super::mem::{span_within, MAX_SPAN};
use super::region::Region;
use super::region_cut::cut;

impl Guest {
    /// Back `[at, at + len)` of a reservation with the given protection, the
    /// commit a fixed mmap makes. Pages the guest has not touched get zeroed
    /// frames; pages it has touched keep their contents, since peer_map skips
    /// a page that is already there. The span is then recorded as backed, in
    /// place of the reservation it came from, so fork copies it.
    pub fn commit(&mut self, at: u64, len: u64, write: bool, exec: bool) -> i64 {
        let mut prot = 0;
        if write {
            prot |= PEER_PROT_WRITE;
        }
        if exec {
            prot |= PEER_PROT_EXEC;
        }
        let mut done = 0;
        while done < len {
            let take = (len - done).min(MAX_SPAN);
            let rc = mk_peer_map(self.pid, at + done, take, prot);
            if rc < 0 {
                return rc;
            }
            done += take;
        }
        self.regions = cut(&self.regions, at, len);
        self.regions.push(Region::new(at, len, write, exec, true));
        0
    }

    /// Take `len` of address space at `addr` without backing it: a PROT_NONE
    /// reservation. Bytes appear, zeroed, when the guest first touches them.
    pub fn reserve(&mut self, addr: u64, len: u64) -> i64 {
        let Some((start, span)) = span_within(addr, len, USER_MAX) else {
            return -1;
        };
        self.regions.push(Region::new(start, span, true, false, false));
        0
    }
}
