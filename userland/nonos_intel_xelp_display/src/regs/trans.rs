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
//! The transcoder's timing and configuration registers. Transcoders A to D
//! sit 0x1000 apart. Tiger Lake has no eDP transcoder: Vol 2c lists no eDP
//! instance of TRANS_HTOTAL (p. 1364), so `Transcoder::Edp` is refused.

use nonos_intel_display_hw::error::Error;
use nonos_intel_display_hw::pipe::{Pipe, Transcoder};

/// TRANS_HTOTAL_A, 60000h (Vol 2c p. 1364).
pub const HTOTAL: u32 = 0x6_0000;
/// TRANS_HBLANK_A, 60004h (p. 1360).
pub const HBLANK: u32 = 0x6_0004;
/// TRANS_HSYNC_A, 60008h (p. 1362).
pub const HSYNC: u32 = 0x6_0008;
/// TRANS_VTOTAL_A, 6000Ch (p. 1396).
pub const VTOTAL: u32 = 0x6_000C;
/// TRANS_VBLANK_A, 60010h (p. 1377).
pub const VBLANK: u32 = 0x6_0010;
/// TRANS_VSYNC_A, 60014h (p. 1392).
pub const VSYNC: u32 = 0x6_0014;
/// PIPE_SRCSZ_A, 6001Ch; one per pipe, not per transcoder (p. 704).
pub const PIPE_SRCSZ: u32 = 0x6_001C;
/// TRANS_CONF_A, 70008h (p. 1340).
pub const CONF: u32 = 0x7_0008;
/// TRANS_CONF bit 31, Transcoder Enable (p. 1341).
pub const CONF_ENABLE: u32 = 1 << 31;
/// TRANS_CONF bit 30, Transcoder State, read only (p. 1341).
pub const CONF_STATE: u32 = 1 << 30;
/// TRANS_CONF bits 22:21, Interlaced Mode; 00b is progressive (p. 1341).
pub const CONF_INTERLACE: u32 = 3 << 21;

/// The distance from one transcoder's or pipe's instance to the next.
const STEP: u32 = 0x1000;

/// `reg_a`, transcoder A's instance of a register, moved to `t`'s.
pub fn of(reg_a: u32, t: Transcoder) -> Result<u32, Error> {
    let n = match t {
        Transcoder::A => 0,
        Transcoder::B => 1,
        Transcoder::C => 2,
        Transcoder::D => 3,
        Transcoder::Edp => return Err(Error::Argument),
    };
    Ok(reg_a + n * STEP)
}

/// PIPE_SRCSZ of pipe `p`.
pub fn srcsz(p: Pipe) -> u32 {
    PIPE_SRCSZ + p.index() * STEP
}
