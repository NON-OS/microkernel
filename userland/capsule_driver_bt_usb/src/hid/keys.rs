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

//! A Bluetooth keyboard's and mouse's input, posted as the USB HID driver
//! posts a boot keyboard's and mouse's: the same codes, flags, layout
//! cycling and repeat, so no application can tell the two apart.

use nonos_bt_hogp::report::Input;
use nonos_keymap::HeldKeys;
use nonos_libc::{INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_REL};

use super::modifier::modifier_bit;
use super::post_wire::send;
use super::repeat::KeyRepeat;

#[derive(Default)]
pub struct Keys {
    pub(super) modifiers: u8,
    pub(super) caps: bool,
    pub(super) held: HeldKeys,
    pub(super) repeat: KeyRepeat,
    /// Events the input ring refused.
    pub failures: u64,
}

impl Keys {
    pub fn on(&mut self, input: Input) {
        let ok = match input {
            Input::KeyDown(u) | Input::KeyUp(u) if modifier_bit(u).is_some() => {
                let bit = modifier_bit(u).unwrap_or(0);
                let down = matches!(input, Input::KeyDown(_));
                self.modifiers = if down { self.modifiers | bit } else { self.modifiers & !bit };
                true
            }
            Input::KeyDown(u) => self.press(u),
            Input::KeyUp(u) => {
                self.repeat.release(u);
                self.post(u, false)
            }
            Input::ButtonDown(n) => send(INPUT_KIND_BUTTON_DOWN, 0, u32::from(n), 0, 0),
            Input::ButtonUp(n) => send(INPUT_KIND_BUTTON_UP, 0, u32::from(n), 0, 0),
            Input::Motion { dx, dy } => send(INPUT_KIND_POINTER_REL, 0, 0, dx.into(), dy.into()),
        };
        self.failures += u64::from(!ok);
    }
}
