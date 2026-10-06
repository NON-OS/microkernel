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

//! The machine's side of [`FifoBus`]: the uncached register window and the
//! kernel clock. Every refusal is logged here by name, once, as it happens.

use super::bus::FifoBus;
use super::session::run_command;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::mmio::{init_window, read32, read8, write8};

/// Only constructed inside [`transact`], whose caller has accepted what
/// writing the part's registers means; that is what lets `write8` below be
/// a safe trait method.
struct MmioBus;

impl FifoBus for MmioBus {
    /// The window is mapped before this bus exists and is never unmapped,
    /// so the error arm cannot be taken. If it were, the byte a floating
    /// bus returns makes every later check fail by name, not succeed.
    fn read8(&mut self, offset: u32) -> u8 {
        read8(offset).unwrap_or(u8::MAX)
    }

    fn read32(&mut self, offset: u32) -> u32 {
        read32(offset).unwrap_or(u32::MAX)
    }

    fn write8(&mut self, offset: u32, value: u8) {
        // SAFETY: eK@nonos.systems - reachable only from `transact`, whose
        // caller owns the command; `run_command` owns the register sequence.
        let _ = unsafe { write8(offset, value) };
    }

    fn now_ms(&mut self) -> u64 {
        crate::time::now_ns() / 1_000_000
    }

    fn relax(&mut self) {
        core::hint::spin_loop();
    }
}

/// Run one command through the FIFO of locality 0.
///
/// # Safety
/// The caller owns what the command means. This owns only the transport.
pub(in crate::security::tpm) unsafe fn transact(
    cmd: &[u8],
    out: &mut [u8],
) -> Result<usize, TpmError> {
    init_window()?;
    run_command(&mut MmioBus, cmd, out).map_err(|fail| {
        crate::log::warn!("[TPM] fifo: {}", fail.as_str());
        fail.error()
    })
}
