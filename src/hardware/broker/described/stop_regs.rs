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

//! Each interface's own way to stop all DMA, from its specification. Every
//! access is 32 bits wide, which each of the three permits for every
//! register, and every wait is bounded.

/// Polls before a stop is reported as not having taken.
const SPINS: u32 = 10_000_000;

fn read(at: u64) -> u32 {
    // SAFETY: `at` is inside the device's first register window, which
    // `register` mapped as Device memory for the kernel's lifetime.
    unsafe { core::ptr::read_volatile(at as *const u32) }
}

fn write(at: u64, value: u32) {
    // SAFETY: as for `read`.
    unsafe { core::ptr::write_volatile(at as *mut u32, value) }
}

fn wait(mut done: impl FnMut() -> bool) -> bool {
    (0..SPINS).any(|_| {
        core::hint::spin_loop();
        done()
    })
}

/// SD Host Controller Simplified Specification 4.20, 2.2.17: Software Reset
/// For All, bit 0 of the byte at 0x2F, resets the controller and ends any
/// transfer; it reads 1 until the reset is done. Written through the dword
/// at 0x2C, keeping the clock and timeout bytes beside it.
pub(super) fn reset_sdhci(regs: u64) -> bool {
    const RESET_ALL: u32 = 1 << 24;
    write(regs + 0x2c, read(regs + 0x2c) | RESET_ALL);
    wait(|| read(regs + 0x2c) & RESET_ALL == 0)
}

/// xHCI 1.2, 5.4.1 and 5.4.2: clearing USBCMD Run/Stop halts the schedule,
/// and USBSTS HCHalted reads 1 once it has. The operational registers sit
/// CAPLENGTH (5.3.1, the first byte) past the base.
pub(super) fn halt_xhci(regs: u64) -> bool {
    const RUN: u32 = 1 << 0;
    const HALTED: u32 = 1 << 0;
    let op = regs + (read(regs) & 0xff) as u64;
    write(op, read(op) & !RUN);
    wait(|| read(op + 4) & HALTED != 0)
}

/// AHCI 1.3.1, 3.1.2: GHC.HR resets the HBA, stopping every port's DMA, and
/// reads 1 until the reset completes.
pub(super) fn reset_ahci(regs: u64) -> bool {
    const HBA_RESET: u32 = 1 << 0;
    write(regs + 0x04, read(regs + 0x04) | HBA_RESET);
    wait(|| read(regs + 0x04) & HBA_RESET == 0)
}
