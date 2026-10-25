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

//! The engine's `Hw` for a capsule: the register window the broker mapped,
//! the kernel's uptime, and the serial console.

use core::ptr::{read_volatile, write_volatile};

use nonos_libc::mk_uptime_ms;

use super::serial_log::SerialLog;
use crate::ufs::env::{Clock, Log, Mmio};

/// The UFSHCI registers, mapped uncached at `base`.
#[derive(Clone, Copy)]
pub struct HostEnv {
    base: u64,
}

impl HostEnv {
    /// # Safety
    /// `base` must start a live broker MMIO grant covering the UFSHCI
    /// register space (the registers this driver touches end at 0xA0), kept
    /// mapped while this is used.
    pub const unsafe fn new(base: u64) -> Self {
        Self { base }
    }
}

impl Mmio for HostEnv {
    fn r32(&self, off: u32) -> u32 {
        unsafe { read_volatile((self.base + off as u64) as *const u32) }
    }
    fn w32(&self, off: u32, v: u32) {
        unsafe { write_volatile((self.base + off as u64) as *mut u32, v) }
    }
}

/// A refused read is the end of time, so every deadline has passed and a
/// broken clock fails a wait instead of holding it.
impl Clock for HostEnv {
    fn now_ms(&self) -> u64 {
        let t = mk_uptime_ms();
        if t < 0 {
            u64::MAX
        } else {
            t as u64
        }
    }
}

impl Log for HostEnv {
    fn line(&self, text: &[u8]) {
        SerialLog.line(text);
    }
}
