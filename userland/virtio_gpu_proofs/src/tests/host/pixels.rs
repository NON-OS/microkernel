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
//! Pixels to the host, and backing to a resource. A transfer copies each
//! row of the rect from the backing at `offset` plus the row times the
//! resource's width in bytes (the reading this model takes of the 2D
//! transfer; unverified), refusing one that would read past the backing.

use super::state::Host;
use super::words::{fits, w};

impl Host {
    pub fn transfer(&mut self, b: &[u8]) -> bool {
        let (x, y, rw, rh) = (w(b, 0), w(b, 4), w(b, 8), w(b, 12));
        let offset = w(b, 16) as u64 | (w(b, 20) as u64) << 32;
        let Some(r) = self.res.get_mut(&w(b, 24)) else { return false };
        let Some((addr, len)) = r.backing.filter(|_| fits(r, b)) else { return false };
        let row = r.w as u64 * 4;
        let last = offset + (rh as u64 - 1) * row + rw as u64 * 4;
        if last > len as u64 {
            return false;
        }
        for j in 0..rh as u64 {
            let src = (addr + offset + j * row) as *const u32;
            let at = ((y as u64 + j) * r.w as u64 + x as u64) as usize;
            for i in 0..rw as usize {
                // SAFETY: inside the backing, checked above; tests keep it
                // mapped while the host may read it.
                r.px[at + i] = unsafe { src.add(i).read_unaligned() };
            }
        }
        true
    }

    pub fn attach(&mut self, b: &[u8]) -> bool {
        let (id, entries) = (w(b, 0), w(b, 4));
        let addr = w(b, 8) as u64 | (w(b, 12) as u64) << 32;
        let len = w(b, 16);
        let Some(r) = self.res.get_mut(&id).filter(|r| r.backing.is_none()) else { return false };
        let ok = entries == 1 && addr != 0 && len != 0;
        if ok {
            r.backing = Some((addr, len));
        }
        ok
    }
}
