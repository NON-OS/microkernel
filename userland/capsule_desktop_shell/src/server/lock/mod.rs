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

//! The lock screen in the shell's trusted band. Locking covers every output
//! with the chrome, opaque, above every window and popup, and tells the
//! router, which then sends the shell every event and draws the pointer only
//! over the field. The passphrase goes to the keyring, which checks it and
//! holds wrong entries back; nothing here compares or keeps it.

pub mod band;
mod check;
mod keep;
mod prompt;
mod router;
mod start;
mod tick;

pub use check::filter;
pub use prompt::{ask_change, ask_first};
pub use start::start;
pub use tick::tick;

use crate::state::lock::geometry::fields;
use crate::state::lock::words;
use crate::state::{Context, NotifyLevel};

/// Lock now: Super+L, the lid, the idle count. Without a passphrase set
/// nothing is locked, and the shell says how to set one.
pub fn lock_now(ctx: &mut Context) {
    if !ctx.lock.offered() {
        ctx.toasts.push(words::SET_FIRST, NotifyLevel::Info, crate::server::toast_clock::now());
        crate::render::sync_toast_layer(ctx);
        return;
    }
    if ctx.lock.screen.covers() {
        return;
    }
    crate::server::handlers::escape_chord::close_chrome(ctx);
    ctx.lock.choose = None;
    cover(ctx);
}

/// The router's auto-lock time passed: lock when a passphrase is set, and
/// say nothing when none is (the person is away).
pub fn lock_idle(ctx: &mut Context) {
    if ctx.lock.offered() && !ctx.lock.screen.covers() {
        crate::server::handlers::escape_chord::close_chrome(ctx);
        ctx.lock.choose = None;
        cover(ctx);
    }
}

/// The lock screen up over the outputs as they are now.
pub(crate) fn cover(ctx: &mut Context) {
    let rid = ctx.issue_request_id();
    ctx.lock.outputs =
        crate::compositor_client::output_rects::output_rects(ctx.ports.compositor_port, rid)
            .unwrap_or_else(|| alloc::vec![(0, 0, ctx.width, ctx.height)]);
    ctx.lock.screen.lock();
    router::tell(ctx);
    crate::server::repaint::repaint(ctx);
    crate::server::grabs::sync(ctx);
}

/// The fields the pointer is drawn over while locked.
pub(crate) fn shown(ctx: &Context) -> alloc::vec::Vec<(u32, u32, u32, u32)> {
    fields(&ctx.lock.outputs, crate::render::ui_font::px(1))
}
