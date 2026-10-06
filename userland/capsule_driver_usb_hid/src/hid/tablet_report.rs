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

//! One absolute pointer report, laid out as QEMU's usb-tablet sends it:
//! buttons, X and Y as little-endian 16-bit positions, then an optional
//! signed wheel step. Pure, so the decode is proven on the host.

use crate::protocol::TABLET_REPORT_MIN;

/// The three buttons a report's first byte carries.
pub const TABLET_BUTTONS: u8 = 0x07;

/// The highest position on either axis. The tablet's logical range is 0 to
/// 0x7FFF, and the input router scales exactly that range onto the screen,
/// so a larger value would land past the screen's edge.
pub const TABLET_ABS_MAX: u16 = 0x7FFF;

/// What one tablet report says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabletReport {
    pub buttons: u8,
    pub x: i32,
    pub y: i32,
    pub wheel: i32,
}

/// The report in `raw`, or None when it is too short to be one. Positions
/// past the logical range are clamped to its top. Bytes past the wheel are
/// not looked at.
pub fn tablet_report(raw: &[u8]) -> Option<TabletReport> {
    if raw.len() < TABLET_REPORT_MIN {
        return None;
    }
    let buttons = raw[0] & TABLET_BUTTONS;
    let x = i32::from(u16::from_le_bytes([raw[1], raw[2]]).min(TABLET_ABS_MAX));
    let y = i32::from(u16::from_le_bytes([raw[3], raw[4]]).min(TABLET_ABS_MAX));
    let wheel = if raw.len() > 5 { raw[5] as i8 as i32 } else { 0 };
    Some(TabletReport { buttons, x, y, wheel })
}
