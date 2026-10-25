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

use nonos_libc::{InputEvent, INPUT_KIND_KEY_UP};

use crate::state::key_targets::key_of;
use crate::state::Context;

use super::pointer::shell_pid;

pub fn route_keyboard(ctx: &mut Context, event: &InputEvent) -> u32 {
    let is_up = event.kind == INPUT_KIND_KEY_UP;
    let key = key_of(event.flags, event.code);
    // A release goes to whoever received the matching press, so a focus change
    // while a key is held cannot strand the key-down in the old window. A press
    // routes to current focus. Resolving the release from the press target also
    // avoids a synchronous WM query on every key-up.
    // The reserved chord and the system keys (volume, power) go to the shell
    // whatever has focus; a release follows its press there through the press
    // table, like any other key.
    let (kind, code, flags) = (event.kind, event.code, event.flags);
    // Ctrl+Alt+T goes to the shell over focus alone, never over a grab or a hold.
    let chord = super::chord::is_reserved_chord(kind, code, flags)
        || super::chord::is_terminal_chord(kind, code, flags);
    let system = super::shell_keys::is_shell_key(event.code);
    // A system key's release with no press on record (the press went before
    // the shell was up) still goes to the shell, never to a window that did
    // not see the key go down.
    let pid = if is_up {
        let pressed_in = ctx.key_targets.take(key).filter(|&p| p != 0);
        match pressed_in {
            Some(pid) => pid,
            None if system => shell_pid(ctx),
            None => fallback_focus(ctx),
        }
    } else if chord || system {
        shell_pid(ctx)
    } else {
        super::focus_now::focus_now(ctx)
    };
    super::key_deliver::deliver_key(ctx, pid, event)
}

fn fallback_focus(ctx: &mut Context) -> u32 {
    if ctx.last_focus_pid != 0 {
        ctx.last_focus_pid
    } else {
        shell_pid(ctx)
    }
}
