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

//! Pointer events to bytes, for programs that asked for them.

use super::keys::Mods;
use crate::term::MouseMode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Button {
    Left,
    Middle,
    Right,
    WheelUp,
    WheelDown,
    /// Motion with no button held.
    None,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MouseKind {
    Press,
    Release,
    Motion,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MouseEvent {
    pub kind: MouseKind,
    pub button: Button,
    /// Cell, 0-based.
    pub col: usize,
    pub row: usize,
    pub mods: Mods,
}

pub(super) fn wanted(ev: &MouseEvent, mode: MouseMode) -> bool {
    let wheel = matches!(ev.button, Button::WheelUp | Button::WheelDown);
    match (mode, ev.kind) {
        (MouseMode::Off, _) => false,
        (MouseMode::Press, k) => k == MouseKind::Press,
        (_, MouseKind::Press | MouseKind::Release) => !(wheel && ev.kind == MouseKind::Release),
        (MouseMode::Drag, MouseKind::Motion) => ev.button != Button::None,
        (MouseMode::Motion, MouseKind::Motion) => true,
        (MouseMode::Click, MouseKind::Motion) => false,
    }
}
