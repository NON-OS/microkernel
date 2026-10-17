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
//! The status byte's two hard moves. The reset a bring-up starts from
//! (G-73): 0 written, then read back until it reads 0, since a reset may
//! complete after the write, and only a completed one has the device drop
//! every resource, backing and scanout a driver before this one left (both
//! as recalled from the virtio specification; unverified). Nothing goes to
//! the device before then. And FAILED on the modern status byte, set on
//! top of what is there.

use nonos_libc::mk_yield;

use crate::constants::{MOD_DEVICE_STATUS, STATUS_FAILED};
use crate::regs::Regs;

/// Reads of the status byte, each after a yield, before giving up.
const RESET_READS: u32 = 4096;

/// # Safety
/// `regs` must map the device's status byte at `status`.
pub unsafe fn reset(regs: Regs, status: usize) -> Result<(), &'static str> {
    regs.w8(status, 0);
    // The device has a turn before each read: the reset is its to finish.
    for _ in 0..RESET_READS {
        if mk_yield() < 0 {
            break;
        }
        if regs.r8(status) == 0 {
            return Ok(());
        }
    }
    Err("virtio-gpu: device reset did not complete")
}

/// # Safety
/// `regs` must map the modern common configuration.
pub unsafe fn failed(regs: Regs, why: &'static str) -> &'static str {
    regs.w8(MOD_DEVICE_STATUS, regs.r8(MOD_DEVICE_STATUS) | STATUS_FAILED);
    why
}
