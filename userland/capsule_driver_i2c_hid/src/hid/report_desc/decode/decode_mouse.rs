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
use super::body::{body, holds};
use crate::hid::report_desc::mouse_layout::MouseLayout;
use crate::hid::report_desc::read_bits::{read_bits, sign_extend};
use crate::hid::report_desc::Field;
use crate::input::sample::MouseSample;

/// A relative axis: two's complement in its size, held to a byte.
fn axis(body: &[u8], f: &Field) -> i8 {
    let v = sign_extend(read_bits(body, f.bit_offset, f.bit_size), f.bit_size);
    v.clamp(i32::from(i8::MIN), i32::from(i8::MAX)) as i8
}

/// `report` is the input report after the two-byte length prefix. None when
/// it is another report id, or too short to hold the located fields.
pub fn decode_mouse(report: &[u8], layout: &MouseLayout) -> Option<MouseSample> {
    let l = layout;
    let body = body(report, l.report_id)?;
    if !holds(body, &[l.x, l.y, l.wheel, l.buttons]) {
        return None;
    }
    let buttons = read_bits(body, l.buttons.bit_offset, l.buttons.bit_size.min(5)) & 0x1F;
    Some(MouseSample {
        buttons: buttons as u8,
        dx: axis(body, &l.x),
        dy: axis(body, &l.y),
        wheel: axis(body, &l.wheel),
    })
}
