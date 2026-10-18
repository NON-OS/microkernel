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

//! The relative mouse report a pad sends in mouse mode, from a Mouse or
//! Pointer application collection: the report of its first relative X, and
//! in it relative Y, the wheel, and buttons 1 to 5 as one run of bits.

use super::field_at::FieldAt;
use super::walk::walk;
use crate::hid::report_desc::{Field, MouseLayout};

const POINTER: u32 = 0x0001_0001;
const MOUSE: u32 = 0x0001_0002;
const X: u32 = 0x0001_0030;
const Y: u32 = 0x0001_0031;
const WHEEL: u32 = 0x0001_0038;
const BUTTON_1: u32 = 0x0009_0001;
const BUTTON_5: u32 = 0x0009_0005;

fn of_mouse(f: &FieldAt) -> bool {
    !f.feature && (f.app == MOUSE || f.app == POINTER)
}

/// The mouse layout of `desc`; empty when the walk refuses the descriptor
/// or it declares no relative pointer.
pub fn parse_mouse(desc: &[u8]) -> MouseLayout {
    let mut id = None;
    let found = walk(desc, &mut |f| {
        if id.is_none() && of_mouse(f) && f.relative && f.usage == X {
            id = Some(f.id);
        }
    });
    // A report too long to read whole is never decoded (long.rs).
    let (Ok(long), Some(id)) = (found, id) else { return MouseLayout::default() };
    if long.has(id) {
        return MouseLayout::default();
    }
    let mut m = MouseLayout { report_id: id, ..MouseLayout::default() };
    let laid = walk(desc, &mut |f| match f.usage {
        _ if !of_mouse(f) || f.id != id => {}
        X if f.relative && !m.x.present() => m.x = f.field(),
        Y if f.relative && !m.y.present() => m.y = f.field(),
        WHEEL if f.relative && !m.wheel.present() => m.wheel = f.field(),
        BUTTON_1..=BUTTON_5 if f.size == 1 => button(&mut m, f),
        _ => {}
    });
    laid.map_or_else(|_| MouseLayout::default(), |_| m)
}

/// Button n's bit, when it lies where the run from button 1's bit puts it.
fn button(m: &mut MouseLayout, f: &FieldAt) {
    let n = f.usage - BUTTON_1;
    let Some(first) = f.at.checked_sub(n) else { return };
    if !m.buttons.present() {
        m.buttons = Field { bit_offset: first, ..f.field() };
    }
    if m.buttons.bit_offset == first {
        m.button_count = m.button_count.max(n as u8 + 1);
        m.buttons.bit_size = u32::from(m.button_count);
    }
}
