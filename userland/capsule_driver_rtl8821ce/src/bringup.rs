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

//! Cold-start stage one: power the MAC on, confirm it answers, and only then
//! let it master the bus (`pwr::power_on_then_master`). This needs no DMA,
//! so it is the first thing a boot on real silicon can report.

use crate::pwr::{power_on_then_master, NoMaster, PowerOn};
use crate::regs::Mmio;

/// Power the MAC on, read a register back, then `master_on`. Returns the
/// chip-id readback, or why the chip may not master the bus.
pub fn probe<M: Mmio>(mmio: &M, master_on: impl FnOnce() -> bool) -> Result<u32, NoMaster> {
    let (id, how) = power_on_then_master(mmio, master_on)?;
    if let PowerOn::WasLeftPowered { cr_after_off } = how {
        crate::status::line(b"[rtl8821ce] chip was left powered; powered off first, cr=");
        crate::status::hex16(cr_after_off as u16);
        crate::status::line(b"\n");
    }
    Ok(id)
}
