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

//! The stream's memory made ready before the codec is programmed: the PCM
//! ring silent, and the position buffer pointed at on the controllers whose
//! playback position is read from it.

use core::ptr::write_bytes;

use nonos_libc::DmaMapOut;

use crate::constants::{DPLBASE, DPLBASE_ENABLE, DPUBASE};
use crate::controller::bdl::{POSBUF_OFFSET, RING_BYTES};
use crate::controller::dma_sync::flush;
use crate::controller::intel;
use crate::discover::Found;
use crate::regs::Regs;

/// The ring starts silent and the engine stays stopped: the first sound is
/// the first period a player writes after OP_STREAM_START. A ring filled at
/// init and left running loops forever, and the codec's outputs stay muted
/// until then too (`program`).
pub(super) fn silence(sample: &DmaMapOut) {
    // SAFETY: the broker mapped the ring's RING_BYTES, page rounded, at
    // user_va for this capsule alone.
    unsafe { write_bytes(sample.user_va as *mut u8, 0, RING_BYTES as usize) };
    flush(sample.user_va, RING_BYTES);
}

/// Point DPLBASE at the position buffer in the BDL page, on the controllers
/// whose playback position is read from it, and return this stream's entry.
pub(super) fn position_buffer(regs: Regs, dev: &Found, bdl: &DmaMapOut, gi: u16) -> Option<u64> {
    if !intel::position_buffer(dev.vendor) {
        return None;
    }
    let va = bdl.user_va + POSBUF_OFFSET;
    let pa = bdl.device_addr + POSBUF_OFFSET;
    // SAFETY: the BDL grant is a whole 4 KiB page and the 256 bytes from
    // POSBUF_OFFSET (0x800) lie inside it; DPLBASE and DPUBASE are global
    // registers of every HDA window.
    unsafe {
        write_bytes(va as *mut u8, 0, 256);
        regs.w32(DPUBASE, (pa >> 32) as u32);
        regs.w32(DPLBASE, pa as u32 | DPLBASE_ENABLE);
    }
    flush(va, 256);
    Some(va + gi as u64 * 8)
}
