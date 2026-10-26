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

//! An SD host's registers as `sdhci_stop` reaches them: one byte at a time,
//! within the span mapped when the host was added, and the kernel's clock.

use super::sdhci_stop::Window;

#[derive(Clone, Copy)]
pub(super) struct Regs {
    va: u64,
    span: u64,
}

impl Regs {
    pub(super) const fn new(va: u64, span: u64) -> Self {
        Self { va, span }
    }
}

impl Window for Regs {
    fn read8(&mut self, off: u64) -> Option<u8> {
        // SAFETY: inside the span `note_sd_host` mapped as device memory for
        // the kernel's lifetime.
        (off < self.span).then(|| unsafe { core::ptr::read_volatile((self.va + off) as *const u8) })
    }

    fn write8(&mut self, off: u64, v: u8) -> bool {
        if off >= self.span {
            return false;
        }
        // SAFETY: as for `read8`.
        unsafe { core::ptr::write_volatile((self.va + off) as *mut u8, v) };
        true
    }

    fn now_ms(&mut self) -> u64 {
        crate::time::timestamp_millis()
    }
}
