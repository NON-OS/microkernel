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

use core::ptr::read_volatile;

use super::super::intid::is_spi;
use super::state::{V2m, USED_WORDS, V2M};

const MSI_TYPER: u64 = 0x008;
const MSI_SETSPI_NS: u64 = 0x040;

/// Take the frame at `base`, raising `count` SPIs from `first`, or, with
/// `count` zero, the ones its `MSI_TYPER` reports. Returns the physical
/// address of the doorbell a device writes an INTID to.
pub fn bring_up(base: u64, first: u32, count: u32) -> Result<u64, &'static str> {
    let (first, count) = if count != 0 {
        (first, count)
    } else {
        // SAFETY: the frame the tree described, mapped as device memory at
        // boot beside the distributor.
        let typer = unsafe { read_volatile((base + MSI_TYPER) as *const u32) };
        ((typer >> 16) & 0x3ff, typer & 0x3ff)
    };
    let last = first.checked_add(count.saturating_sub(1));
    if count == 0 || !is_spi(first) || !last.is_some_and(is_spi) {
        return Err("the GICv2m frame's range is not SPIs");
    }
    let mut v2m = V2M.lock();
    if v2m.is_some() {
        return Err("a GICv2m frame is already in service");
    }
    *v2m = Some(V2m { first, count, used: [0; USED_WORDS] });
    Ok(base + MSI_SETSPI_NS)
}
