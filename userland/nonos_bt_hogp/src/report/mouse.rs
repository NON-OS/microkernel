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

//! The boot mouse report (HID 1.11, B.2): three button bits, then signed X
//! and Y motion. Further octets are the device's own and are not read.

use super::input::Input;
use alloc::vec::Vec;

const BUTTONS: u8 = 0x07;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Mouse {
    buttons: u8,
}

impl Mouse {
    /// The events in one report; a report under three octets changes
    /// nothing.
    pub fn report(&mut self, raw: &[u8]) -> Vec<Input> {
        let &[b, x, y, ..] = raw else { return Vec::new() };
        self.moved(b, i16::from(x as i8), i16::from(y as i8), 0)
    }

    /// Motion, then the wheel, then each of buttons 1 to 3 that changed;
    /// a report-mode pointer comes here too, once its map is read.
    pub fn moved(&mut self, buttons: u8, dx: i16, dy: i16, wheel: i16) -> Vec<Input> {
        let mut out = Vec::new();
        if dx != 0 || dy != 0 {
            out.push(Input::Motion { dx, dy });
        }
        if wheel != 0 {
            out.push(Input::Wheel(wheel));
        }
        out.extend(self.buttons_to(buttons & BUTTONS));
        out
    }

    fn buttons_to(&mut self, now: u8) -> Vec<Input> {
        let was = core::mem::replace(&mut self.buttons, now);
        (0..3u8)
            .filter(|i| (was ^ now) >> i & 1 == 1)
            .map(
                |i| {
                    if now >> i & 1 == 1 {
                        Input::ButtonDown(i + 1)
                    } else {
                        Input::ButtonUp(i + 1)
                    }
                },
            )
            .collect()
    }

    /// Releases for every button held, for when the link goes.
    pub fn release_all(&mut self) -> Vec<Input> {
        self.buttons_to(0)
    }
}
