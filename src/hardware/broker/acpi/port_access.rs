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

//! The `in` and `out` the kernel runs for the ACPI capsule, each on a port
//! `io_grant::port` has just checked.

use super::error::RegionError;
use super::io_grant::port;
use crate::arch::port_io::{inb, inl, inw, outb, outl, outw};
use core::sync::atomic::{AtomicBool, Ordering};

static TOLD_SLEEP_ENABLE: AtomicBool = AtomicBool::new(false);

pub fn port_read(pid: u32, at: u64, width: u8) -> Result<u32, RegionError> {
    let p = port(pid, at, width)?;
    // SAFETY: the caller holds the port and the rule allows it, so it belongs
    // to no driver and to none of the devices the kernel runs itself.
    Ok(unsafe {
        match width {
            1 => u32::from(inb(p)),
            2 => u32::from(inw(p)),
            _ => inl(p),
        }
    })
}

pub fn port_write(pid: u32, at: u64, width: u8, value: u32) -> Result<(), RegionError> {
    let p = port(pid, at, width)?;
    if super::kernel::power_regs::sets_sleep_enable(at, width, value) {
        // Once: AML retrying the write cannot fill the log.
        if !TOLD_SLEEP_ENABLE.swap(true, Ordering::Relaxed) {
            crate::sys::serial::println(b"[ACPI] refused: a port write setting SLP_EN");
        }
        return Err(RegionError::SleepEnable);
    }
    // SAFETY: as for `port_read`; the value is cut to the width asked for.
    unsafe {
        match width {
            1 => outb(p, value as u8),
            2 => outw(p, value as u16),
            _ => outl(p, value),
        }
    }
    Ok(())
}
