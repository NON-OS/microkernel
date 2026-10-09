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

use crate::clients::wm;
use crate::state::Context;

use super::deliver::deliver_one;
use super::pointer::shell_pid;

pub fn route_keyboard(ctx: &mut Context, event: &InputEvent) -> u32 {
    let is_up = event.kind == INPUT_KIND_KEY_UP;
    // A release goes to whoever received the matching press, so a focus change
    // while a key is held cannot strand the key-down in the old window. A press
    // routes to current focus. Resolving the release from the press target also
    // avoids a synchronous WM query on every key-up.
    // The reserved chord and the system keys (volume, power) go to the shell
    // whatever has focus; a release follows its press there through the press
    // table, like any other key.
    let chord = super::chord::is_reserved_chord(event.kind, event.code, event.flags);
    let system = super::shell_keys::is_shell_key(event.code);
    // A system key's release with no press on record (the press went before
    // the shell was up) still goes to the shell, never to a window that did
    // not see the key go down.
    let pid = if is_up {
        let pressed_in = ctx.key_targets.take(event.code).filter(|&p| p != 0);
        match pressed_in {
            Some(pid) => pid,
            None if system => shell_pid(ctx),
            None => fallback_focus(ctx),
        }
    } else if chord || system {
        shell_pid(ctx)
    } else {
        let rid = ctx.issue_request_id();
        let answer = wm::query_focus(&mut ctx.wm_port, rid);
        if let Some(focused) = answer {
            ctx.last_focus_pid = focused;
        }
        let shell = shell_pid(ctx);
        super::key_target::press_target(answer, ctx.last_focus_pid, shell)
    };
    // A held key repeats as more presses. One that now goes to another window
    // than the press before it is first released where it went down: that
    // window would otherwise hold the key for good, and a client that repeats
    // held keys itself (a Linux app under the Wayland bridge) types it forever.
    if !is_up {
        if let Some(held) = ctx.key_targets.held_by(event.code).filter(|&p| p != pid) {
            ctx.key_targets.take(event.code);
            release_in(ctx, held, event);
        }
    }
    if !ctx.subscriptions.allows(pid, event.kind) {
        ctx.record(0);
        return 0;
    }
    let delivered = deliver_one(pid, event);
    if !is_up && delivered != 0 {
        ctx.key_targets.remember(event.code, pid);
    }
    if delivered == 0 {
        ctx.forget_pid(pid);
    }
    ctx.record(delivered);
    delivered
}

fn release_in(ctx: &mut Context, pid: u32, press: &InputEvent) {
    let mut release = *press;
    release.kind = INPUT_KIND_KEY_UP;
    if ctx.subscriptions.allows(pid, release.kind) && deliver_one(pid, &release) == 0 {
        ctx.forget_pid(pid);
    }
}

fn fallback_focus(ctx: &mut Context) -> u32 {
    if ctx.last_focus_pid != 0 {
        ctx.last_focus_pid
    } else {
        shell_pid(ctx)
    }
}
