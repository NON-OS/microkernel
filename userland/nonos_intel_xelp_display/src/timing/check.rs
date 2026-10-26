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
//! The limits Vol 2c sets on a transcoder's timing, all checked before any
//! register is written. Progressive modes only: an interlaced mode is
//! refused until interlaced output has its own unit. HDMI audio's wider
//! blank (138 pixels, p. 1360) is not checked because no audio is sent.

use nonos_edid::Detailed;
use nonos_intel_display_hw::error::Error;

/// What the transcoder drives, where the limits differ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sink {
    DisplayPort,
    Hdmi,
}

/// One past the largest value of a 14-bit horizontal field (TRANS_HTOTAL
/// 29:16 and 13:0, p. 1364) and of a 13-bit vertical one (TRANS_VTOTAL
/// 28:16 and 12:0, p. 1396); PIPE_SRCSZ's width is 13 bits (28:16, p. 704).
const H_FIELD: u64 = 1 << 14;
const V_FIELD: u64 = 1 << 13;
const SRC_WIDTH: u64 = 1 << 13;
/// Source heights above 4320 lines are refused: PIPE_SRCSZ takes no more (p. 705).
const SRC_HEIGHT_MAX: u64 = 4320;

/// `Ok` when every limit below holds for `t` on `sink`.
pub fn check(t: &Detailed, sink: Sink) -> Result<(), Error> {
    let hdmi = sink == Sink::Hdmi;
    let (ha, hb, hf, hs) = (t.h_active as u64, t.h_blank as u64, t.h_front as u64, t.h_sync as u64);
    let (va, vb, vf, vs) = (t.v_active as u64, t.v_blank as u64, t.v_front as u64, t.v_sync as u64);
    let ok = !t.interlaced
        && t.pixel_khz > 0
        // Active at least 64 pixels, 256 on HDMI (TRANS_HTOTAL 13:0, p. 1365).
        && ha >= if hdmi { 256 } else { 64 }
        // The source width is even (PIPE_SRCSZ 28:16, p. 705).
        && ha % 2 == 0
        // Blank at least 32 pixels (TRANS_HBLANK 29:16, p. 1360).
        && hb >= 32
        // Sync starts past the active field's last pixel, and on HDMI at
        // least 16 pixels past blank start (TRANS_HSYNC 13:0, p. 1363).
        && hf >= if hdmi { 16 } else { 1 }
        // Sync ends after it starts and before the total (29:16, p. 1363).
        && hs >= 1
        && hf + hs < hb
        // At least one line; blank at least 5 lines (TRANS_VBLANK, p. 1378).
        && va >= 1
        && vb >= 5
        // Vertical sync likewise inside the blank (TRANS_VSYNC, p. 1393).
        && vf >= 1
        && vs >= 1
        && vf + vs < vb
        && ha + hb <= H_FIELD
        && va + vb <= V_FIELD
        && ha <= SRC_WIDTH
        && va <= SRC_HEIGHT_MAX;
    if ok {
        Ok(())
    } else {
        Err(Error::Argument)
    }
}
