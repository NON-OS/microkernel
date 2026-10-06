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

//! The wheel reads history, or on the alternate screen, where there is
//! none, sends the cursor keys a pager scrolls by.

use nonos_app_skeleton::{EventOutcome, InputEvent};
use nonos_vt::input::{encode_key, Key, Mods};

use crate::event::send_to_program;
use crate::term::terminal::Terminal;

const WHEEL_LINES: usize = 3;

impl Terminal {
    pub(super) fn wheel(&mut self, event: InputEvent) -> EventOutcome {
        let state = self.cur();
        let vt = &state.scrollback.vt;
        let up = event.delta_y > 0;
        let lines = (event.delta_y.unsigned_abs() as usize).clamp(1, 10) * WHEEL_LINES;
        if vt.alt_active() {
            if state.fg_running && vt.modes.alt_scroll {
                let key = if up { Key::Up } else { Key::Down };
                let mut bytes = alloc::vec::Vec::new();
                for _ in 0..lines {
                    encode_key(key, Mods::default(), &vt.modes, &mut bytes);
                }
                send_to_program(state, &bytes);
            }
            return EventOutcome::Idle;
        }
        if up {
            state.scrollback.scroll_up(lines);
        } else {
            state.scrollback.scroll_down(lines);
        }
        EventOutcome::Repaint
    }
}
