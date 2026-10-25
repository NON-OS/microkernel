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

//! The registers by name. A command is a write to CMD; a GET command's
//! answer is the next read of CMD, and the capsule is one thread, which is
//! what keeps the two together.

use super::mmio::Mmio;
use crate::constants::regs::{CMD, DSAH, DSAL, MACH, MACL, UVRS, VRRS};

#[derive(Clone, Copy)]
pub struct Regs {
    pub(super) pt: Mmio,
    pub(super) vd: Mmio,
}

// SAFETY anchor for every unsafe here and in doorbell.rs: `pt` and `vd` map
// BAR0 and BAR1 whole (setup::mmio asks for 4 KiB of each), and every
// offset named in constants::regs is a register inside its page.
impl Regs {
    pub const fn new(pt_base: u64, vd_base: u64) -> Self {
        Self { pt: Mmio::new(pt_base), vd: Mmio::new(vd_base) }
    }

    /// The revisions offered, then select `want`; false if it is not offered.
    pub fn select(&self, want: u32) -> bool {
        let offered = unsafe { (self.vd.r32(VRRS), self.vd.r32(UVRS)) };
        if offered.0 & want == 0 || offered.1 & want == 0 {
            return false;
        }
        unsafe {
            self.vd.w32(VRRS, want);
            self.vd.w32(UVRS, want);
        }
        true
    }

    pub fn command(&self, cmd: u32) -> u32 {
        unsafe {
            self.vd.w32(CMD, cmd);
            self.vd.r32(CMD)
        }
    }

    pub fn set_shared(&self, pa: u64) {
        unsafe {
            self.vd.w32(DSAL, pa as u32);
            self.vd.w32(DSAH, (pa >> 32) as u32);
        }
    }

    pub fn set_mac(&self, m: &[u8; 6]) {
        unsafe {
            self.vd.w32(MACL, u32::from_le_bytes([m[0], m[1], m[2], m[3]]));
            self.vd.w32(MACH, u32::from_le_bytes([m[4], m[5], 0, 0]));
        }
    }
}
