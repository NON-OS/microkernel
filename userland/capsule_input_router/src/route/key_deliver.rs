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

//! A key event delivered to the process chosen for it, keeping the press
//! table: a press is remembered so its release follows it, a release takes
//! its entry, and a held key's repeat that now goes elsewhere is first let
//! go where it went down. That window would otherwise hold the key for
//! good, and a client that repeats held keys itself (a Linux app under the
//! Wayland bridge) types it forever.

use nonos_libc::{InputEvent, INPUT_KIND_KEY_UP};

use crate::state::Context;

use super::deliver::deliver_one;

pub fn deliver_key(ctx: &mut Context, pid: u32, event: &InputEvent) -> u32 {
    let is_up = event.kind == INPUT_KIND_KEY_UP;
    if is_up {
        ctx.key_targets.take(event.code);
    } else if let Some(held) = ctx.key_targets.held_by(event.code).filter(|&p| p != pid) {
        ctx.key_targets.take(event.code);
        release_in(ctx, held, event);
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
