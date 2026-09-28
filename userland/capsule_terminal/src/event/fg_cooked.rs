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

//! Keys for a program reading lines: the line is edited here and sent on
//! Enter, as a tty's canonical mode does.

use nonos_app_skeleton::EventOutcome;
use nonos_vt::input::{Key, Mods};

use super::cooked::Effect;
use super::fg_input::send;
use super::interrupt::interrupt;
use crate::term::state::State;

pub(super) fn cooked(state: &mut State, key: Key, m: Mods) -> EventOutcome {
    let mut fx = Effect::default();
    let lc = |c: char| c.to_ascii_lowercase();
    match key {
        Key::Char(c) if m.ctrl => match lc(c) {
            'c' => return interrupt(state),
            'u' => state.cooked.kill_line(&mut fx),
            'w' => state.cooked.kill_word(&mut fx),
            'd' if state.cooked.line.is_empty() => {
                /*
                 * There is no end-of-input for a NONOS program's stdin yet;
                 * saying so beats a key that silently does nothing.
                 */
                fx.echo.extend_from_slice(b"^D (end of input is not delivered to this program)\n");
            }
            _ => return EventOutcome::Idle,
        },
        Key::Char(c) => state.cooked.char(c, &mut fx),
        Key::Tab => state.cooked.char('\t', &mut fx),
        Key::Backspace => state.cooked.backspace(&mut fx),
        Key::Enter => state.cooked.enter(&mut fx),
        _ => return EventOutcome::Idle,
    }
    if !fx.echo.is_empty() {
        state.scrollback.feed_raw(&fx.echo);
    }
    if !fx.send.is_empty() {
        send(state, &fx.send);
    }
    EventOutcome::Repaint
}
