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

//! One boot mouse report: buttons, then signed X and Y motion, then an
//! optional signed wheel step. Pure, so the decode is proven on the host.

use crate::protocol::MOUSE_REPORT_MIN;

use super::mouse_event::MouseEvent;

/// The five buttons a report's first byte carries; its top three bits are
/// padding.
pub const MOUSE_BUTTONS: u8 = 0x1f;

/// The event `report` makes against the buttons held before it, or None
/// when it is too short to be a report or reports nothing new. Bytes past
/// the wheel are not looked at.
pub fn mouse_event(report: &[u8], previous_buttons: u8) -> Option<MouseEvent> {
    if report.len() < MOUSE_REPORT_MIN {
        return None;
    }
    let buttons = report[0] & MOUSE_BUTTONS;
    let dx = report[1] as i8 as i16;
    let dy = report[2] as i8 as i16;
    let dz = if report.len() > 3 { report[3] as i8 } else { 0 };
    let moved = dx != 0 || dy != 0;
    let changed = buttons != previous_buttons;
    if !moved && dz == 0 && !changed {
        return None;
    }
    let flags = u8::from(moved) | (u8::from(changed) << 1) | (u8::from(dz != 0) << 2);
    Some(MouseEvent { dx, dy, dz, buttons, flags })
}
