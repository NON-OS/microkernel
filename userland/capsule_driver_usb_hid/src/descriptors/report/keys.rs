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

//! The keys of a Keyboard or Keypad collection, in the shape HID 1.11
//! appendix B.1 gives: a bit for each modifier, usages 0xE0 to 0xE7, and
//! slots of key usages given as a range. A bitmap (n-key rollover) keyboard
//! is not read.

use super::input::Input;
use super::layout::{Slots, NONE};

const KEYBOARD_PAGE: u32 = 0x0007_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Keys {
    pub id: u8,
    pub app: u16,
    /// Modifier i, usage 0xE0 + i, is the bit at mods[i].
    pub mods: [u16; 8],
    pub slots: Option<Slots>,
    /// The keyboard usage a slot value of `slots.min` names.
    pub first: u16,
}

impl Keys {
    pub fn usable(&self) -> bool {
        self.slots.is_some()
    }
}

pub fn take(part: &mut Option<Keys>, inp: &Input) {
    if inp.variable {
        for i in 0..inp.g.count {
            let usage = inp.l.usage(i);
            if inp.g.size != 1 || !(KEYBOARD_PAGE | 0xE0..=KEYBOARD_PAGE | 0xE7).contains(&usage) {
                continue;
            }
            let Some(k) = mine(part, inp) else { return };
            let bit = &mut k.mods[(usage & 7) as usize];
            if *bit == NONE {
                *bit = inp.field_at(i);
            }
        }
        return;
    }
    let keyboard = inp.l.range().filter(|&(lo, _)| lo & 0xFFFF_0000 == KEYBOARD_PAGE);
    let (Some((lo, _)), Some(slots)) = (keyboard, inp.slots()) else { return };
    let Some(k) = mine(part, inp) else { return };
    if k.slots.is_none() {
        k.slots = Some(slots);
        k.first = lo as u16;
    }
}

/// The part, made on its first field, when `inp` is of its report and its
/// collection.
fn mine<'a>(part: &'a mut Option<Keys>, inp: &Input) -> Option<&'a mut Keys> {
    let new = Keys { id: inp.id, app: inp.app, mods: [NONE; 8], slots: None, first: 0 };
    let k = part.get_or_insert(new);
    (k.id == inp.id && k.app == inp.app).then_some(k)
}
