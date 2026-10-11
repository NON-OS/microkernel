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

//! The NVRAM arbiter, taken before a chip reset so the bootcode is not
//! reading NVRAM when the core clock stops (tg3_nvram_lock). The reset
//! itself drops the lock, so it is never released by hand on that path.

use crate::constants::grc::{NVRAM_SWARB, SWARB_GNT1, SWARB_REQ_CLR1, SWARB_REQ_SET1};
use crate::constants::timeouts::NVRAM_LOCK_MS;
use crate::init::wait;
use crate::regs::Regs;

/// Whether the arbiter granted it. tg3_chip_reset goes on either way.
pub fn lock(regs: &Regs) -> bool {
    // SAFETY: `regs` carries the broker MmioMap base for BAR0, NVRAM_SWARB in it.
    unsafe { regs.w32(NVRAM_SWARB, SWARB_REQ_SET1) };
    let granted = wait::until(NVRAM_LOCK_MS, || unsafe { regs.r32(NVRAM_SWARB) & SWARB_GNT1 != 0 });
    if !granted {
        unsafe { regs.w32(NVRAM_SWARB, SWARB_REQ_CLR1) };
    }
    granted
}
