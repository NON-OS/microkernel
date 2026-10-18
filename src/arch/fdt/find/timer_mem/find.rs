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

//! One usable frame of the memory-mapped timer (the Arm generic timer
//! binding, `arm,armv7-timer-mem`, recalled, unverified): the node's `reg`
//! is CNTCTLBase, and each `frame@N` child has a `frame-number`, a `reg`
//! whose first entry is that frame's CNTBaseN, and `interrupts` whose first
//! is the frame's physical timer. A frame firmware keeps for itself is
//! marked disabled.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::irq::gic_intid;
use crate::arch::fdt::parser::Fdt;

const NEEDLES: &[&[u8]] = &[b"arm,armv7-timer-mem"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerFrame {
    /// `frame-number`, 0 to 7.
    pub number: u32,
    /// CNTBaseN, translated to a CPU physical address, and its size.
    pub base: u64,
    pub size: u64,
    /// The frame's physical timer as a GIC INTID.
    pub intid: u32,
}

/// The first enabled frame with a number, a register window and a physical
/// timer interrupt, in tree order. A frame missing any of them is passed
/// over; one whose values are damaged refuses the read.
pub fn find(fdt: &Fdt) -> Result<Option<TimerFrame>, FdtError> {
    let Some(found) = fdt.devices(NEEDLES).next() else {
        return Ok(None);
    };
    let (timer, _) = found?;
    for frame in fdt.children(&timer)? {
        let frame = frame?;
        if !frame.named(b"frame") || !fdt.is_enabled(&frame)? {
            continue;
        }
        let number = match fdt.property(&frame, b"frame-number")? {
            Some(p) if p.data.len() == 4 => p.u32()?,
            Some(_) => return Err(FdtError::BadCells),
            None => continue,
        };
        if number > 7 {
            return Err(FdtError::BadCells);
        }
        let (Some((base, size)), Some(irq)) = (fdt.mmio(&frame, 0)?, fdt.interrupt(&frame, 0)?)
        else {
            continue;
        };
        return Ok(Some(TimerFrame { number, base, size, intid: gic_intid(irq.cells)? }));
    }
    Ok(None)
}
