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

//! Backing part of a reservation with pages, as an mprotect that asks for
//! access does.

use super::handle::Guest;
use super::mem::maps_full;
use super::mem_span::map_span;
use super::region::{peer_prot, Region};
use super::region_cut::cut;

impl Guest {
    /// Back `[at, at + len)` of a reservation with the given protection, the
    /// commit an mprotect that asks for access makes. The kernel fills no page
    /// a guest touches on its own, so every page here is new and zeroed. The
    /// span is then recorded as backed, in place of the reservation it came
    /// from, so fork copies it.
    pub fn commit(&mut self, at: u64, len: u64, write: bool, exec: bool) -> i64 {
        /* A commit inside a reservation splits it in three. */
        if maps_full(self.regions.len(), self.regions.len() + 2) {
            return -1;
        }
        let rc = map_span(self.pid, at, len, peer_prot(write, exec, true));
        if rc < 0 {
            return rc;
        }
        let kept = self.span_kept(at, len);
        self.regions = cut(&self.regions, at, len);
        self.regions.push(Region {
            at,
            len,
            write,
            exec,
            access: true,
            unproven: false,
            backed: true,
            kept,
        });
        0
    }
}
