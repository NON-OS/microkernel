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
use crate::hid::report_desc::layout::Field;
use crate::hid::report_desc::mouse_layout::MouseLayout;
use crate::hid::report_desc::read_bits::read_bits;
use crate::input::sample::MouseSample;

/// A signed field: HID relative axes are two's complement in their size.
fn signed(body: &[u8], f: &Field) -> i32 {
    if !f.present() || f.bit_size > 32 {
        return 0;
    }
    let raw = read_bits(body, f.bit_offset, f.bit_size);
    let shift = 32 - f.bit_size;
    ((raw << shift) as i32) >> shift
}

fn clamp8(v: i32) -> i8 {
    v.clamp(i32::from(i8::MIN), i32::from(i8::MAX)) as i8
}

/// `report` is the input report after the two-byte length prefix. None when
/// it is another report id, or too short to hold the located fields.
pub fn decode_mouse(report: &[u8], layout: &MouseLayout) -> Option<MouseSample> {
    let body = if layout.report_id != 0 {
        if report.first().copied()? != layout.report_id {
            return None;
        }
        &report[1..]
    } else {
        report
    };
    let need = [layout.x, layout.y, layout.wheel, layout.buttons]
        .iter()
        .filter(|f| f.present())
        .map(|f| f.bit_offset + f.bit_size)
        .max()
        .unwrap_or(0);
    if (body.len() as u32) * 8 < need {
        return None;
    }
    let buttons = if layout.buttons.present() {
        (read_bits(body, layout.buttons.bit_offset, layout.buttons.bit_size.min(5)) & 0x1F) as u8
    } else {
        0
    };
    Some(MouseSample {
        buttons,
        dx: clamp8(signed(body, &layout.x)),
        dy: clamp8(signed(body, &layout.y)),
        wheel: clamp8(signed(body, &layout.wheel)),
    })
}
