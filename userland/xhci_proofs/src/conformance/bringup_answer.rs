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

//! The controller's answer to the bring-up's NOOP: one Command Completion
//! Event, written where the segment table the driver programmed says the
//! event ring is, naming the first TRB of the command ring CRCR names.

use nonos_devmodel::FakeBar;
use nonos_libc::dma_host;

use super::model::{wrote64, CAP_LEN};
use crate::constants::{CRCR_LO, ERSTBA_LO, TRB_TYPE_CMD_COMPLETION_EVENT};
use crate::trb::{read_volatile_at, write_volatile_at, Trb};

/// The primary interrupter: the runtime registers at 0x1000, past MFINDEX.
const INTERRUPTER0: usize = 0x1000 + 0x20;

/// Answer the first command with `code`, once the rings are programmed.
/// Returns whether the answer was written.
pub fn answer_first_command(bar: &FakeBar, code: u8) -> bool {
    let Some(erst) = dma_host(wrote64(bar, INTERRUPTER0 + ERSTBA_LO as usize)) else {
        return false;
    };
    // SAFETY: `dma_host` named live host memory behind ERSTBA.
    let entry = unsafe { read_volatile_at(erst) };
    let Some(segment) = dma_host(entry.d0 as u64 | (entry.d1 as u64) << 32) else {
        return false;
    };
    let ring = wrote64(bar, CAP_LEN + CRCR_LO as usize) & !0x3F;
    let d3 = (TRB_TYPE_CMD_COMPLETION_EVENT << 10) | 1;
    let event = Trb { d0: ring as u32, d1: (ring >> 32) as u32, d2: (code as u32) << 24, d3 };
    // SAFETY: `dma_host` named live host memory behind the segment.
    unsafe { write_volatile_at(segment, event) };
    true
}
