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

//! The media keys of a Consumer Control or System Control collection: Mute,
//! Volume Up and Down, and Power, each either a bit of its own or a value
//! some array slot holds while the key is down. A keyboard sends these on
//! an interface of their own, so without it they did nothing over USB.

use super::input::Input;
use super::layout::{Slots, NONE};

/// Each media key as (usage page << 16 | usage), with the keyboard usage
/// the driver posts it as, which the keycode map already gives a code.
pub const MEDIA_KEYS: [(u32, u8); 4] =
    [(0x000C_00E2, 0x7F), (0x000C_00E9, 0x80), (0x000C_00EA, 0x81), (0x0001_0081, 0x66)];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Media {
    pub id: u8,
    pub app: u16,
    /// MEDIA_KEYS[i] is the bit at bits[i].
    pub bits: [u16; 4],
    pub slots: Option<Slots>,
    /// MEDIA_KEYS[i] is down while a slot holds values[i].
    pub values: [Option<i64>; 4],
}

impl Media {
    pub fn usable(&self) -> bool {
        self.bits.iter().any(|&b| b != NONE) || self.values.iter().any(Option::is_some)
    }
}

pub fn take(part: &mut Option<Media>, inp: &Input) {
    if inp.variable {
        for i in 0..inp.g.count {
            let usage = inp.l.usage(i);
            let key = MEDIA_KEYS.iter().position(|&(u, _)| u == usage);
            let Some(key) = key.filter(|_| inp.g.size == 1) else { continue };
            let Some(m) = mine(part, inp) else { return };
            if m.bits[key] == NONE {
                m.bits[key] = inp.field_at(i);
            }
        }
        return;
    }
    let value = |usage| inp.l.index_of(usage).map(|i| inp.g.min + i64::from(i));
    let values = MEDIA_KEYS.map(|(usage, _)| value(usage).filter(|&v| v <= inp.g.max));
    let Some(slots) = inp.slots().filter(|_| values.iter().any(Option::is_some)) else { return };
    let Some(m) = mine(part, inp) else { return };
    if m.slots.is_none() {
        m.slots = Some(slots);
        m.values = values;
    }
}

fn mine<'a>(part: &'a mut Option<Media>, inp: &Input) -> Option<&'a mut Media> {
    let new = Media { id: inp.id, app: inp.app, bits: [NONE; 4], slots: None, values: [None; 4] };
    let m = part.get_or_insert(new);
    (m.id == inp.id && m.app == inp.app).then_some(m)
}
