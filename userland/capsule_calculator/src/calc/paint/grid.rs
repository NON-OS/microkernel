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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Button, Kind, State as Look};

use crate::calc::buttons::Role;
use crate::calc::event::hover::look;
use crate::calc::hit::Hit;
use crate::calc::mode::Mode;
use crate::calc::prog::allowed;
use crate::calc::state::State;
use crate::calc::ui::keypad_geom::keys;

/// The kind of button each role of key draws as. Equals leads; an operator
/// is a quiet accent label, which keeps it apart from the digits; the digits,
/// functions and memory keys are ordinary buttons.
fn kind(role: Role) -> Kind {
    match role {
        Role::Equals => Kind::Primary,
        Role::Operator => Kind::Plain,
        Role::Number | Role::Function | Role::Memory => Kind::Secondary,
    }
}

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width as i32, fb.height as i32);
    for (row, idx, btn, at) in keys(state.mode, w, h) {
        let enabled = state.mode != Mode::Programmer || allowed(state.base, btn.action);
        let st = Look { disabled: !enabled, ..look(state, Hit::Key(row, idx)) };
        Button::new(btn.label).kind(kind(btn.role)).state(st).draw(fb, at);
    }
}
