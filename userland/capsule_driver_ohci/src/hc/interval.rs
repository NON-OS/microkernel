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
//! The frame interval, kept across the host reset as firmware tuned it,
//! and the largest data packet that fits a frame of it. FI is read from
//! the controller, so it is held to a frame a full-speed bus can have:
//! 12000 bit times give or take 0.25%, the full-speed rate's tolerance
//! (USB 2.0 7.1.11, as recalled). Anything else, 0 (no tuning) among it,
//! is the reset value 11999 (0x2edf, Linux's FI in ohci.h).

use crate::facts::regs::{FI_DEFAULT, FI_MASK};

/// How far firmware's tuning may move FI from FI_DEFAULT.
pub const FI_SLACK: u32 = 30;

/// FI from HcFmInterval as read, or FI_DEFAULT when it is no frame.
pub fn frame_interval(reg: u32) -> u32 {
    let fi = reg & FI_MASK;
    match fi.abs_diff(FI_DEFAULT) <= FI_SLACK {
        true => fi,
        false => FI_DEFAULT,
    }
}

/// FSMPS: the frame less the bit-stuffing and SOF overhead, 6/7 of
/// FI - 210 (spec, as recalled); none for a frame shorter than that.
pub fn largest_packet(fi: u32) -> u32 {
    6 * fi.saturating_sub(210) / 7
}
