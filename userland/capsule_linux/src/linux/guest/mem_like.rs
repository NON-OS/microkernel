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

//! A span that takes after another: what mremap makes when a mapping grows
//! or moves, since Linux gives the new part the mapping's own protection.

use super::handle::Guest;
use super::mem::{maps_full, MAX_SPAN};
use super::mem_span::map_span;
use super::region::Region;

impl Guest {
    /// `[at, at + len)` held like `like`: the same protection, provenance and
    /// backing. A reservation stays one, with no pages.
    pub fn map_like(&mut self, at: u64, len: u64, like: &Region) -> i64 {
        if maps_full(self.regions.len(), self.regions.len() + 1) {
            return -1;
        }
        if like.backed {
            let rc = map_span(self.pid, at, len, like.peer_prot());
            if rc < 0 {
                return rc;
            }
        }
        self.regions.push(Region { at, len, ..*like });
        0
    }

    /// Copy `len` bytes from `from` to `to` inside the guest, a megabyte at a
    /// time. The kernel copies whatever the pages' protection is.
    pub fn copy_within(&self, from: u64, to: u64, len: u64) -> bool {
        let mut done = 0;
        while done < len {
            let take = (len - done).min(MAX_SPAN);
            let Some(bytes) = self.read(from + done, take as usize) else {
                return false;
            };
            if self.write(to + done, &bytes) < bytes.len() as i64 {
                return false;
            }
            done += take;
        }
        true
    }
}
