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
//! The service thread's side of the first paint. While the worker is on it
//! (`ctx.first_paint_out`) the service thread writes nothing to the backing
//! and asks the compositor for no place in its scene, so nothing half drawn
//! is shown and nothing is drawn under the worker. It answers every call
//! meanwhile: a call that would paint waits for the worker first (`wait`).

use nonos_libc::{mk_debug, mk_yield, Look};

use super::job::{paint, Backing};
use super::worker::{poll, send, EMBEDDED};
use crate::server::scene::first_submit;
use crate::state::Context;

/// At the end of setup: hand the painting to a worker, or paint here when
/// none can start, as setup did before.
pub fn start(ctx: &mut Context) {
    let b =
        Backing { va: ctx.backing_va, stride: ctx.stride, width: ctx.width, height: ctx.height };
    if send(b) {
        ctx.first_paint_out = true;
        return;
    }
    say(b"[WALLPAPER] no worker for the built-in picture; painting it here\n");
    finish(ctx, paint(b, EMBEDDED));
}

/// Each turn of the service loop: take the worker's answer once it is in.
pub fn step(ctx: &mut Context) {
    if !ctx.first_paint_out {
        return;
    }
    match poll() {
        Look::Waiting => {}
        Look::Ready(shown) => finish(ctx, shown),
        // Not reached: a worker out leaves the slot Running until it puts.
        Look::Idle => finish(ctx, false),
    }
}

/// Before a request paints the backing: wait for the worker to be done.
pub fn wait(ctx: &mut Context) {
    while ctx.first_paint_out {
        step(ctx);
        mk_yield();
    }
}

/// The backing is the service thread's again: ask the compositor to show
/// it, and the loop asks again if it does not answer (server/scene).
fn finish(ctx: &mut Context, shown: bool) {
    ctx.first_paint_out = false;
    if !shown {
        say(b"[WALLPAPER] the built-in picture did not decode; ink shown\n");
    }
    first_submit(ctx);
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
