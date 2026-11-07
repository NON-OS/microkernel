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

use alloc::vec;

use nonos_libc::{mk_input_event_wait, mk_uptime_ms, InputEvent};

use crate::clients::compositor;
use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::route::route_event;
use crate::sources::{drain_batch, kernel_ring::MAX_BATCH, stamp};
use crate::state::policy_turn::Step;
use crate::state::Context;

use super::drain_ipc::drain_ipc;

// Upper bound on how long the router parks when the kernel input ring is empty.
// A posted event wakes it immediately; this only bounds the case where the wake
// signal is missed, so keeping it well under a frame stops a click from waiting
// on the timeout before it is routed to the focused window.
const INPUT_WAIT_MS: u64 = 8;

pub fn run() -> ! {
    let mut ctx = Context::new();
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut batch: [InputEvent; MAX_BATCH] = [InputEvent::default(); MAX_BATCH];
    let mut last_seq: u64 = 0;
    let mut sweep_ticks: u32 = 0;
    ctx.idle.input(mk_uptime_ms());
    loop {
        drain_ipc(&mut ctx, &mut rx, &mut tx);
        sweep_ticks = sweep_ticks.wrapping_add(1);
        if sweep_ticks & 0x3f == 0 {
            ctx.purge_dead();
        }
        let n = drain_batch(&mut batch);
        stamp(&mut batch[..n], mk_uptime_ms());
        for ev in batch.iter().take(n) {
            route_event(&mut ctx, ev);
        }
        if ctx.cursor_dirty {
            let rid = ctx.issue_request_id();
            let (x, y) = (ctx.cursor_x, ctx.cursor_y);
            let shown = crate::route::session_lock::pointer_shown(
                ctx.session_locked,
                &ctx.session_shown,
                x,
                y,
            );
            let _ = compositor::cursor_update(&mut ctx.compositor_port, rid, x, y, shown);
            ctx.cursor_dirty = false;
        }
        read_policy(&mut ctx, n > 0);
        tell_idle(&mut ctx);
        tell_auto_lock(&mut ctx);
        if n == 0 {
            crate::route::settle_hover(&mut ctx);
            let _ = mk_input_event_wait(last_seq, INPUT_WAIT_MS, &mut last_seq);
        }
    }
}

/// The screen blank as idle wants it now, told to the compositor until it
/// took it (state/idle.rs).
fn tell_idle(ctx: &mut Context) {
    let now = mk_uptime_ms();
    let want = ctx.idle.step(now);
    if ctx.idle.due(want, now) {
        let rid = ctx.issue_request_id();
        let took = compositor::idle_blank(&mut ctx.compositor_port, rid, want);
        ctx.idle.answered(want, now, took);
    }
}

/// The policy store's next setting, when one is due and this pass `routed`
/// no input (state/policy_turn.rs). A setting the store does not give is
/// kept as it was; a store that did not answer (its port forgotten) ends
/// the round.
fn read_policy(ctx: &mut Context, routed: bool) {
    let now = mk_uptime_ms();
    let Some(step) = ctx.policy.take(now, routed) else { return };
    let port = &mut ctx.policy_port;
    match step {
        Step::Sensitivity => {
            if let Some(v) = crate::clients::policy::mouse_sensitivity(port) {
                ctx.cursor.mult_x2 = v.clamp(1, 4) as i32;
            }
        }
        Step::ScreenTimeout => {
            if let Some(m) = crate::clients::policy::screen_timeout(port) {
                ctx.idle.set_timeout_minutes(m);
            }
        }
        Step::AutoLock => read_auto_lock(ctx),
    }
    let answered = ctx.policy_port != 0;
    ctx.policy.read(now, answered);
}

/// The policy's auto-lock time and its choice for the idle inhibit; kept
/// as they were when the store does not answer.
fn read_auto_lock(ctx: &mut Context) {
    let port = &mut ctx.policy_port;
    let Some(minutes) = crate::clients::policy::auto_lock_timeout(port) else { return };
    let holds =
        crate::clients::policy::lock_inhibit_holds(port).unwrap_or(ctx.auto_lock.inhibit_holds);
    ctx.auto_lock.set(minutes, holds);
}

/// The shell told once the auto-lock time passed with no input
/// (state/idle/auto_lock.rs); told again on the next pass when it did not
/// take it.
fn tell_auto_lock(ctx: &mut Context) {
    let now = mk_uptime_ms();
    let (last, inhibited) = (ctx.idle.last_input_ms, ctx.idle.inhibited());
    if !ctx.auto_lock.due(last, now, inhibited, ctx.session_locked) {
        return;
    }
    let shell = crate::clients::wire::lookup_pid(b"desktop_shell").unwrap_or(0);
    if crate::route::notice::tell_idle_lock(shell) {
        ctx.auto_lock.told(last);
    }
}
