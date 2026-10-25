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

//! Taking the tree's timer frame into use, once the GIC and the kernel's own
//! mappings are up.

use core::arch::asm;

use super::frame::Frame;
use super::handler::on_fire;
use super::state::{DESCRIBED, FRAME, OWNER};
use crate::arch::aarch64::boot::multicore::roster;
use crate::arch::aarch64::cpu::mpidr_affinity;
use crate::arch::aarch64::gic::{enable_irq, register_irq_handler, route_spi_here};
use crate::arch::aarch64::timer::{current_count, frequency};
use crate::memory::addr::PhysAddr;
use crate::memory::mmio::map_device_memory;

/// Map the frame, check it counts the counter this CPU reads, and take its
/// interrupt. The frame's number, or why there is none to use.
pub fn start() -> Result<u32, &'static [u8]> {
    let found = DESCRIBED.get().ok_or(&b"the device tree names no memory-mapped timer"[..])?;
    let size = found.size.max(0x1000) as usize;
    let va = map_device_memory(PhysAddr::new(found.base), size)
        .map_err(|_| &b"its frame could not be mapped"[..])?;
    let frame = Frame::new(va.as_u64());
    frame.hold(None);
    // The same system counter, or a deadline from one means nothing to the
    // other: the frame's count, read between two of this CPU's, lies
    // between them. Each read completes before the next starts.
    let before = current_count();
    // SAFETY: barriers only.
    unsafe { asm!("isb", options(nostack)) };
    let seen = frame.count();
    unsafe { asm!("dsb sy", "isb", options(nostack)) };
    let after = current_count();
    if seen < before || seen > after {
        return Err(b"its frame does not count with this CPU's counter");
    }
    let rate = frame.frequency();
    if rate != 0 && u64::from(rate) != frequency() {
        return Err(b"its frame reports another counter frequency");
    }
    register_irq_handler(found.intid, on_fire).map_err(|_| &b"its interrupt is taken"[..])?;
    // Aimed here, at the CPU that owns it from now on, not left wherever
    // the GIC's bring-up happened to route every SPI.
    let owner = roster::index_of(mpidr_affinity()).ok_or(&b"this CPU is not in the roster"[..])?;
    route_spi_here(found.intid).map_err(str::as_bytes)?;
    OWNER.call_once(|| owner);
    FRAME.call_once(|| frame);
    enable_irq(found.intid);
    Ok(found.number)
}
