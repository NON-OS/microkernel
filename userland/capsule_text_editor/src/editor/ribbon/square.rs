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

//! A square ribbon control as the shared `IconButton`, lit by the ribbon's
//! `Pointer`. Its hover and press come from the same column a click on it
//! lands in, so it lights only where a click would run it.

use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{IconButton, Kind, Pointer, Rect, State, Surface};

/// The side of a square's icon in logical pixels: the ribbon's old marks
/// were this wide, a little under a toolbar's `ICON`.
pub const MARK: u32 = 16;

/// One square control: the column a click on it lands in, the body drawn
/// inside it, its icon, whether the value it names is in force, and whether
/// the model backs it.
#[derive(Clone, Copy)]
pub struct Square {
    pub band: Rect,
    pub body: Rect,
    pub icon: IconId,
    pub on: bool,
    pub live: bool,
}

impl Square {
    /// The hover and press the pointer gives it; a control the model does
    /// not back takes neither and draws as disabled.
    pub fn state(&self, pointer: &Pointer) -> State {
        if !self.live {
            return State { disabled: true, ..State::default() };
        }
        pointer.state(|x, y| contains(self.band, x, y))
    }

    /// The button it draws as: `Primary`, the choice in force, while its
    /// value holds, else `Plain`.
    pub fn button(&self, pointer: &Pointer) -> IconButton<'static> {
        let kind = if self.on { Kind::Primary } else { Kind::Plain };
        IconButton::new(self.icon).icon_size(MARK).kind(kind).state(self.state(pointer))
    }

    /// Draws the button over its body.
    pub fn draw(&self, fb: &mut impl Surface, pointer: &Pointer) {
        self.button(pointer).draw(fb, self.body);
    }
}
