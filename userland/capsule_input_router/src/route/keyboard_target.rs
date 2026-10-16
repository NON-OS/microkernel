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

/// Where a key event goes: a release to its press's target, a reserved or
/// system key to the shell, any other press to focus.
pub(super) fn target(
    ctx: &mut Context,
    event: &InputEvent,
    is_up: bool,
    to_shell: bool,
    system: bool,
) -> u32 {
    if is_up {
        let pressed_in = ctx.key_targets.take(event.code).filter(|&p| p != 0);
        match pressed_in {
            Some(pid) => pid,
            None if system => shell_pid(ctx),
            None => fallback_focus(ctx),
        }
    } else if to_shell {
        shell_pid(ctx)
    } else {
        let rid = ctx.issue_request_id();
        let answer = wm::query_focus(&mut ctx.wm_port, rid);
        if let Some(focused) = answer {
            ctx.last_focus_pid = focused;
        }
        let shell = shell_pid(ctx);
        super::key_target::press_target(answer, ctx.last_focus_pid, shell)
    }
}

pub(super) fn release_in(ctx: &mut Context, pid: u32, press: &InputEvent) {
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
