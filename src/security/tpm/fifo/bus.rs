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

//! What the FIFO protocol needs from the machine, and the bounded waits.
//!
//! The protocol is written against this trait rather than against the
//! window so a host test can put a modelled register file behind it and
//! drive the same code that runs on the machine.

use super::fail::FifoFail;
use super::regs::{STS_BURST_MASK, STS_BURST_SHIFT, TIMEOUT_A_MS, TPM_STS};

pub(crate) trait FifoBus {
    fn read8(&mut self, offset: u32) -> u8;
    fn read32(&mut self, offset: u32) -> u32;
    fn write8(&mut self, offset: u32, value: u8);
    /// A clock that only moves forward. Every wait below ends when it has
    /// moved far enough, so no wait depends on the part to end it.
    fn now_ms(&mut self) -> u64;
    /// Called between polls.
    fn relax(&mut self) {}
}

/// Poll `offset` until the bits in `mask` read as `want`, for at most `ms`.
/// Returns the byte that satisfied the wait so the caller reads the other
/// bits of the same sample rather than of a later one.
pub(crate) fn wait_bits<B: FifoBus>(
    bus: &mut B,
    offset: u32,
    mask: u8,
    want: u8,
    ms: u64,
) -> Option<u8> {
    let start = bus.now_ms();
    loop {
        let value = bus.read8(offset);
        if value & mask == want {
            return Some(value);
        }
        if bus.now_ms().saturating_sub(start) > ms {
            return None;
        }
        bus.relax();
    }
}

/// How many bytes the FIFO takes or gives next, waiting for it to be
/// nonzero. The whole register is read at once: some parts do not decode a
/// 16-bit read of the burst count at offset 0x19.
pub(crate) fn burst_count<B: FifoBus>(bus: &mut B) -> Result<usize, FifoFail> {
    let start = bus.now_ms();
    loop {
        let burst = (bus.read32(TPM_STS) >> STS_BURST_SHIFT) & STS_BURST_MASK;
        if burst != 0 {
            return Ok(burst as usize);
        }
        if bus.now_ms().saturating_sub(start) > TIMEOUT_A_MS {
            return Err(FifoFail::NoBurst);
        }
        bus.relax();
    }
}
