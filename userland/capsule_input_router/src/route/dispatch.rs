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

use nonos_libc::{mk_uptime_ms, InputEvent};

use crate::state::Context;

use super::held::deliver_held;
use super::key_deliver::deliver_key;
use super::kinds::{is_keyboard, is_pointer, key_facts};
use super::preempt::{preempt, Route};
use super::press_kinds::{is_pad, is_pad_button};
use super::{bond, keyboard, pointer, refuse, trusted_hold};

pub fn route_event(ctx: &mut Context, event: &InputEvent) -> u32 {
    let now = mk_uptime_ms();
    let hold = trusted_hold::holder(ctx.trusted, now);
    if hold.is_none() {
        ctx.trusted = None;
    }
    let key_like = is_keyboard(event.kind) || is_pad_button(event.kind);
    let key = key_like.then(|| key_facts(ctx, event));
    let shell = pointer::shell_pid(ctx);
    let route = preempt(key, hold, shell);
    if bond::unproven(event.flags) && refuse::refused(ctx, event, (hold, route, shell), now) {
        return 0;
    }
    match route {
        Route::To(pid) => return deliver_key(ctx, pid, event),
        Route::Held(h) if key.is_some() => return deliver_key(ctx, h, event),
        Route::Held(h) => {
            let (n, pressed) = deliver_held(ctx, h, event);
            ctx.record(n + pressed);
            return n + pressed;
        }
        Route::Normal => {}
    }
    if let Some(holder) = ctx.grabs.holder_for(event.kind) {
        let (n, pressed) = deliver_held(ctx, holder, event);
        if n == 0 {
            // A holder that cannot receive its grabbed stream (inbox full or
            // dead) must not keep the grab: the lost event may be the very
            // release edge that would have ended it, and a grab that outlives
            // its release swallows all input forever. Routing normally again
            // is strictly better than a permanent wedge.
            ctx.grabs.release(holder);
            ctx.forget_pid(holder);
        }
        ctx.record(n + pressed);
        return n + pressed;
    }
    if is_pointer(event.kind) {
        return pointer::route_pointer(ctx, event);
    }
    if is_keyboard(event.kind) {
        return keyboard::route_keyboard(ctx, event);
    }
    if is_pad(event.kind) {
        return super::pad::route_pad(ctx, event);
    }
    super::fanout::fan_out(ctx, event)
}
