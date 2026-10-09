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

//! Pointer events for a foreground program that asked for mouse reports.

use nonos_app_skeleton::{InputEvent, InputKind, MOD_SHIFT};
use nonos_vt::input::{encode_mouse, Button, MouseEvent, MouseKind};
use nonos_vt::MouseMode;

use super::select::{BTN_LEFT, BTN_RIGHT};
use crate::event::keymap::mods;
use crate::event::send_to_program;
use crate::term::terminal::Terminal;

impl Terminal {
    /// Hand the event to the program when it asked for mouse reports and
    /// Shift is not held. True when the program took it.
    pub(super) fn program_mouse(&mut self, event: &InputEvent) -> bool {
        let Some(g) = self.cells else { return false };
        let state = self.cur_ref();
        let vt = &state.scrollback.vt;
        if !state.fg_running || vt.modes.mouse == MouseMode::Off || event.flags & MOD_SHIFT != 0 {
            return false;
        }
        let (col, row) = g.cell_at(vt, event.x, event.y);
        let (kind, button) = match event.kind {
            InputKind::ButtonDown | InputKind::ButtonUp => {
                let b = match event.code {
                    BTN_LEFT => Button::Left,
                    BTN_RIGHT => Button::Right,
                    _ => Button::Middle,
                };
                let down = event.kind == InputKind::ButtonDown;
                self.ptr.held = down.then_some(b);
                (if down { MouseKind::Press } else { MouseKind::Release }, b)
            }
            InputKind::PointerAbs => (MouseKind::Motion, self.ptr.held.unwrap_or(Button::None)),
            InputKind::Wheel if event.delta_y > 0 => (MouseKind::Press, Button::WheelUp),
            InputKind::Wheel => (MouseKind::Press, Button::WheelDown),
            _ => return false,
        };
        let ev = MouseEvent { kind, button, col, row, mods: mods(event.flags) };
        let mut bytes = alloc::vec::Vec::new();
        let state = self.cur();
        if encode_mouse(&ev, &state.scrollback.vt.modes, &mut bytes) {
            send_to_program(state, &bytes);
        }
        true
    }
}
