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

//! Reading the RIRB: every entry the controller has written since the last
//! read, the unsolicited ones counted and set aside.

use core::ptr::read_volatile;

use super::dma_sync::flush;
use super::verb::Link;
use crate::constants::{RIRBWP, RIRB_EX_CODEC, RIRB_EX_UNSOL};

impl Link {
    /// Consume every RIRB entry the controller has written. The first
    /// solicited answer from `cad` is returned; once it is found the rest
    /// stay for the next call.
    pub(super) fn drain(&mut self, cad: Option<u8>) -> Option<u32> {
        // SAFETY: RIRBWP lies in the register window.
        let hw = unsafe { self.regs.r16(RIRBWP) };
        if hw == 0xffff {
            // A controller that dropped off the bus reads all ones.
            return None;
        }
        let hw = hw & self.mask;
        while self.rp != hw {
            self.rp = (self.rp + 1) & self.mask;
            let at = self.rirb_va + self.rp as u64 * 8;
            flush(at, 8);
            // SAFETY: `rp` is masked to the ring, at most 256 entries of 8
            // bytes, all inside the RIRB grant.
            let (resp, ex) =
                unsafe { (read_volatile(at as *const u32), read_volatile((at + 4) as *const u32)) };
            if ex & RIRB_EX_UNSOL != 0 {
                self.unsolicited = self.unsolicited.saturating_add(1);
                continue;
            }
            if let Some(c) = cad {
                if (ex & RIRB_EX_CODEC) as u8 == c {
                    return Some(resp);
                }
            }
        }
        None
    }
}
