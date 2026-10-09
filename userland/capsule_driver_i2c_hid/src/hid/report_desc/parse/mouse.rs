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

//! The relative mouse collection of a report descriptor: the report a pad
//! sends while it is in mouse mode (HID boot-mouse shape or wider). Every
//! touchpad carries one, and it is what the pad reports when it refuses the
//! precision touchpad input mode or has no touch collection at all, so the
//! fields are located exactly rather than guessed from the bytes.

use super::mouse_walk::walk_inputs;
use crate::hid::report_desc::layout::Field;
use crate::hid::report_desc::mouse_layout::MouseLayout;

const PAGE_GENERIC_DESKTOP: u16 = 0x01;
const PAGE_BUTTON: u16 = 0x09;
const USAGE_X: u16 = 0x30;
const USAGE_Y: u16 = 0x31;
const USAGE_WHEEL: u16 = 0x38;
const INPUT_RELATIVE: u32 = 1 << 2;
const INPUT_CONSTANT: u32 = 1 << 0;

/// Locate the relative mouse report: the report holding the first relative
/// X, and in it relative Y, the wheel and the button bits.
pub fn parse_mouse(desc: &[u8]) -> MouseLayout {
    let mut rid = None;
    walk_inputs(desc, |fl| {
        let rel = fl.flags & INPUT_RELATIVE != 0 && fl.flags & INPUT_CONSTANT == 0;
        if rid.is_none() && rel && fl.page == PAGE_GENERIC_DESKTOP && fl.usage == USAGE_X {
            rid = Some(fl.report_id);
        }
    });
    let mut m = MouseLayout::default();
    let Some(rid) = rid else { return m };
    m.report_id = rid;
    walk_inputs(desc, |fl| {
        if fl.report_id != rid || fl.flags & INPUT_CONSTANT != 0 {
            return;
        }
        let field = Field { bit_offset: fl.bit_offset, bit_size: fl.bit_size, logical_max: 0 };
        match (fl.page, fl.usage) {
            (PAGE_GENERIC_DESKTOP, USAGE_X) if !m.x.present() => m.x = field,
            (PAGE_GENERIC_DESKTOP, USAGE_Y) if !m.y.present() => m.y = field,
            (PAGE_GENERIC_DESKTOP, USAGE_WHEEL) if !m.wheel.present() => m.wheel = field,
            (PAGE_BUTTON, 1..=5) if fl.bit_size == 1 => {
                if !m.buttons.present() {
                    m.buttons = Field { bit_size: 0, ..field };
                    m.buttons.bit_offset =
                        fl.bit_offset - (u32::from(fl.usage) - 1).min(fl.bit_offset);
                }
                m.button_count = m.button_count.max(fl.usage as u8);
                m.buttons.bit_size = u32::from(m.button_count);
            }
            _ => {}
        }
    });
    m
}
