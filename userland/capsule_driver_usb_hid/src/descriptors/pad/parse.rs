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
//! A gamepad's layout read from its report descriptor (HID 1.11, 6.2.2,
//! as recalled, unverified): the global state (page, logical range, report
//! size, count and ID, with Push and Pop) and the usages given before each
//! main item, kept for the Input items inside a top-level Joystick or Game
//! Pad application. A descriptor with neither, or with no field read here,
//! is no gamepad.

use alloc::vec::Vec;

use super::input::{input, Globals};
use super::items::items;
use super::layout::{PadLayout, GAME_PAD, JOYSTICK, PAGE_DESKTOP};
use super::usages::{usage, Usages};

/// Push nests no deeper than this; a descriptor that does is no gamepad.
const STACK: usize = 8;

pub fn pad_layout(report: &[u8]) -> Option<PadLayout> {
    let (mut g, mut stack, mut u) = (Globals::default(), Vec::new(), Usages::default());
    let (mut depth, mut in_pad, mut seen) = (0u32, false, false);
    let (mut layout, mut offsets) = (PadLayout::default(), Vec::new());
    for it in items(report) {
        match it.tag {
            0x04 => g.page = it.value as u16,
            0x14 => g.min = it.signed(),
            0x24 => g.max = it.signed(),
            0x74 => g.size = it.value,
            0x84 => (g.id, layout.ids) = (it.value as u8, true),
            0x94 => g.count = it.value,
            0xA4 if stack.len() < STACK => stack.push(g),
            0xA4 => return None,
            0xB4 => g = stack.pop()?,
            0x08 => u.list.push(usage(&it, g.page)),
            0x18 => u.min = Some(usage(&it, g.page)),
            0x28 => u.max = Some(usage(&it, g.page)),
            0xA0 => {
                if it.value == 1 && depth == 0 {
                    in_pad = matches!(u.list.last(), Some(&(PAGE_DESKTOP, JOYSTICK | GAME_PAD)));
                    seen |= in_pad;
                }
                depth += 1;
            }
            0xC0 => {
                depth = depth.saturating_sub(1);
                in_pad &= depth > 0;
            }
            0x80 => input(&g, &u, it.value, in_pad, &mut offsets, &mut layout),
            _ => {}
        }
        // A main item ends the usages given for it.
        if it.tag & 0x0C == 0 {
            u = Usages::default();
        }
    }
    (seen && !layout.fields.is_empty()).then_some(layout)
}
