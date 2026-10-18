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

//! Carrying a request out, as VMMDev.cpp does: the header checked, the
//! body's size checked exactly for its type, nothing but the host version
//! and the guest info before the guest info, then the request itself.

use nonos_libc::dma_at;

use super::host::Host;

const VERR_INVALID_PARAMETER: i32 = -2;
const VERR_NOT_SUPPORTED: i32 = -37;

/// Each request type this model answers, with its whole size.
fn size_of(kind: u32) -> Option<usize> {
    Some(match kind {
        1 | 2 => 36,
        4 => 40,
        41 => 28,
        42 | 56 | 50 => 32,
        _ => return None,
    })
}

impl Host {
    pub fn request(&mut self, addr: u32) {
        if self.opt.deaf {
            return;
        }
        let p = dma_at(addr as u64) as *mut u32;
        // SAFETY: `dma_at` found the request inside a live grant.
        let word = |i: usize| unsafe { u32::from_le(*p.add(i)) };
        let (size, version, kind) = (word(0) as usize, word(1), word(2));
        let body: Vec<u32> = (0..4).map(|i| word(6 + i)).collect();
        let rc = match size_of(kind) {
            _ if version != 0x0001_0001 => VERR_INVALID_PARAMETER,
            Some(want) if want != size => VERR_INVALID_PARAMETER,
            None => VERR_NOT_SUPPORTED,
            Some(_) if self.info.is_none() && kind != 4 && kind != 50 => VERR_NOT_SUPPORTED,
            Some(_) => self.carry_out(kind, &body, p),
        };
        // SAFETY: as above; the status word is inside the header.
        unsafe { *p.add(3) = (rc as u32).to_le() };
        self.requests.push((kind, rc));
    }
}
