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
//! One full-duplex transfer, a byte out for each byte in, in lock step: a
//! frame is queued only once the one before it has come back, so neither
//! FIFO can overrun whatever its depth. Chip select is held for the whole
//! transfer. Every wait is bounded by `time`, which says whether time is
//! left.

use super::Regs32;
use crate::facts::{SSDR, SSSR, SSSR_BSY, SSSR_RNE, SSSR_ROR, SSSR_TNF};

pub const E_TIMEDOUT: i32 = -110;
pub const E_IO: i32 = -5;
/// More frames than any SSP FIFO holds (as recalled, unverified).
const STALE_MAX: usize = 256;

/// `tx` sent while `rx` (as long) fills; the core must be enabled.
pub fn exchange<R: Regs32>(
    r: &R,
    tx: &[u8],
    rx: &mut [u8],
    mut time: impl FnMut() -> bool,
) -> Result<(), i32> {
    if tx.len() != rx.len() {
        return Err(E_IO);
    }
    // A frame left from an aborted transfer would shift every byte after;
    // a receive flag that never clears is a dead core, not a full FIFO.
    let mut drained = 0;
    while r.read32(SSSR) & SSSR_RNE != 0 {
        if drained == STALE_MAX {
            return Err(E_IO);
        }
        let _ = r.read32(SSDR);
        drained += 1;
    }
    r.write32(SSSR, SSSR_ROR);
    for (out, slot) in tx.iter().zip(rx.iter_mut()) {
        wait(r, SSSR_TNF, true, &mut time)?;
        r.write32(SSDR, *out as u32);
        wait(r, SSSR_RNE, true, &mut time)?;
        *slot = r.read32(SSDR) as u8;
    }
    wait(r, SSSR_BSY, false, &mut time)?;
    if r.read32(SSSR) & SSSR_ROR != 0 {
        r.write32(SSSR, SSSR_ROR);
        return Err(E_IO);
    }
    Ok(())
}

fn wait<R: Regs32>(r: &R, bit: u32, set: bool, time: &mut impl FnMut() -> bool) -> Result<(), i32> {
    loop {
        if (r.read32(SSSR) & bit != 0) == set {
            return Ok(());
        }
        if !time() {
            return Err(E_TIMEDOUT);
        }
    }
}
