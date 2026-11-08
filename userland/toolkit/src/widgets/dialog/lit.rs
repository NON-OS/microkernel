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

//! Which button is focused, hovered and held, and the `Button` each is drawn
//! as.

use super::Dialog;
use crate::decorations::Rect;
use crate::widgets::{Button, Kind, Pointer, State};

impl<'a> Dialog<'a> {
    /// The button the keyboard is on, which draws `focused`. An index past
    /// the last button rings none.
    pub fn focus(mut self, index: usize) -> Self {
        self.focus = Some(index);
        self
    }

    /// The button under the pointer, which draws `hovered`.
    pub fn hovered(mut self, index: Option<usize>) -> Self {
        self.hovered = index;
        self
    }

    /// The button held down, which draws `pressed`.
    pub fn pressed(mut self, index: Option<usize>) -> Self {
        self.pressed = index;
        self
    }

    /// Whether the dialog is the second step of a destructive action, so
    /// every `Kind::Danger` button draws armed, filled with `danger`, as a
    /// `Button` with `State::selected` does. It changes no other button and
    /// no size, place or hit. Unarmed by default.
    pub fn armed(mut self, on: bool) -> Self {
        self.armed = on;
        self
    }

    /// Button `index` draws disabled, as a `Button` with `State::disabled`
    /// does, takes no hover or press and is never hit, while the others act
    /// as before. It changes no size or place. An index past the last
    /// disables none, and none is disabled by default.
    pub fn disable(mut self, index: usize) -> Self {
        self.off = Some(index);
        self
    }

    /// Whether button `i` is disabled, alone or with the dialog.
    pub(super) fn off(&self, i: usize) -> bool {
        self.state.disabled || self.off == Some(i)
    }

    /// The hovered and held buttons as `pointer` gives them for the dialog
    /// drawn at `at`: each button takes `Pointer::state` over a hit test
    /// that names it, so a press dragged off one button holds none.
    pub fn pointer(self, pointer: &Pointer, at: Rect) -> Self {
        let each = |i: usize| pointer.state(|x, y| self.hit(at, x, y) == Some(i));
        let lit = |f: fn(&State) -> bool| (0..self.buttons.len()).find(|&i| f(&each(i)));
        self.hovered(lit(|s| s.hovered)).pressed(lit(|s| s.pressed))
    }

    /// The state the app reports for the whole dialog. Only `disabled` is
    /// read: it draws the text and every button in `disabled`, and nothing
    /// is hit. Hover, press and focus belong to one button, so they come
    /// from `hovered`, `pressed` and `focus`.
    pub fn state(mut self, s: State) -> Self {
        self.state = s;
        self
    }

    /// Button `i` as it is drawn: its label and kind, focused, hovered and
    /// held as set, disabled with the dialog or by `disable`, and selected
    /// when it is a `Danger` one in an armed dialog. `None` past the last.
    pub(super) fn button(&self, i: usize) -> Option<Button<'a>> {
        let &(label, kind) = self.buttons.get(i)?;
        let on = |set: Option<usize>| set == Some(i);
        let (focused, hovered, pressed) = (on(self.focus), on(self.hovered), on(self.pressed));
        let (disabled, selected) = (self.off(i), self.armed && kind == Kind::Danger);
        let state = State { focused, hovered, pressed, disabled, selected };
        Some(Button::new(label).kind(kind).state(state))
    }
}
