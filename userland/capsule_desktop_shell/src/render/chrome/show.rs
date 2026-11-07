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
//! A frame just drawn, shown: each surface that flips (state/ctx/flip.rs)
//! has the backing the frame went to submitted in place of the shown one,
//! which the compositor takes in one step. A refused submit leaves the shown
//! frame on screen; the next repaint draws and submits again.
//!
//! A frame that changed only part of the screen (the menu bar's clock each
//! second, the dock) is presented in place of the shown backing with that
//! part named, so only it is composed again: the whole frame is drawn either
//! way, and the rest of it is what was shown. A submit of a surface the
//! whole screen wide made the compositor compose every layer each second.
//! A whole frame is presented the same way, with the band of rows it
//! changed from the shown one named (render/changed_rows.rs), so a whole
//! repaint that changed the dock composes the dock's rows, not the screen.
//! A present the compositor refuses (the chrome went to another band since
//! it was shown, or no room to hold the old backing) is a submit instead.
//!
//! While the lock screen is up the chrome is its curtain, every pixel opaque
//! (render/lock/screen.rs; text blended over an opaque pixel stays opaque),
//! and its present says so, so the compositor composes nothing of a window
//! under it that goes on drawing.

use crate::compositor_client::{push_present_layer, push_scene_submit};
use crate::render::changed_rows::band;
use crate::setup::prime::bands::DESK_Z;
use crate::state::ctx::flip::Flip;
use crate::state::trusted_rule::{CHROME_Z, TRUSTED_Z};
use crate::state::Context;

use super::sent::{sent, Painted, Sent};

/// The part of the screen a frame changed: the whole of it, or a box.
#[derive(Clone, Copy)]
pub enum Changed {
    Whole,
    Part(u32, u32, u32, u32),
}

/// The first row a surface's new frame changed and the row past the last.
pub(super) type Rows = Option<(u32, u32)>;

/// `desk` and `chrome`: whether each surface's frame is new; one that is
/// not is the same as the one on screen, and is not shown. `desk_rows` and
/// `chrome_rows`: the rows a new whole frame changed.
/// What became of each, which says whether a damage commit is owed (sent.rs).
pub(super) fn show(
    ctx: &mut Context,
    changed: Changed,
    (desk, chrome): (bool, bool),
    (desk_rows, chrome_rows): (Rows, Rows),
) -> Painted {
    let desk_at = shown_part(changed, desk_rows, ctx.width);
    let desk = sent(desk, ctx.desk.flips(), || flip(ctx, ctx.desk, (DESK_Z, false), desk_at));
    if desk.taken() {
        ctx.desk.flipped();
    }
    let chrome_z = if ctx.trusted.band.up { TRUSTED_Z } else { CHROME_Z };
    let chrome_at = shown_part(changed, chrome_rows, ctx.width);
    let curtain = ctx.lock.screen.covers();
    let chrome =
        sent(chrome, ctx.chrome.flips(), || flip(ctx, ctx.chrome, (chrome_z, curtain), chrome_at));
    if chrome.taken() {
        ctx.chrome.flipped();
    }
    Painted { desk, chrome }
}

/// The part a new frame is presented with: the caller's, or for a whole
/// frame the band of rows it changed from the one on screen, the screen
/// wide. A whole frame with no such band (drawn on one backing) is submitted
/// whole.
fn shown_part(changed: Changed, rows: Rows, width: u32) -> Changed {
    match (changed, band(rows, width)) {
        (Changed::Whole, Some((x, y, w, h))) => Changed::Part(x, y, w, h),
        _ => changed,
    }
}

/// `f`'s next backing shown in band `z`: presented with the part changed,
/// and as opaque when `opaque`, else submitted whole; Unsaid when the
/// compositor took neither.
fn flip(ctx: &mut Context, f: Flip, (z, opaque): (u32, bool), changed: Changed) -> Sent {
    if let Changed::Part(x, y, w, h) = changed {
        let rid = ctx.issue_request_id();
        let (port, handles) = (ctx.ports.compositor_port, (f.next_handle(), f.shown_handle()));
        if push_present_layer(port, rid, handles, z, (x, y, w, h), opaque).is_ok() {
            return Sent::Presented;
        }
    }
    if submit(ctx, f.next_handle(), z) {
        Sent::Submitted
    } else {
        Sent::Unsaid
    }
}

fn submit(ctx: &mut Context, handle: u64, z: u32) -> bool {
    let rid = ctx.issue_request_id();
    let rect = (0, 0, ctx.width, ctx.height);
    push_scene_submit(ctx.ports.compositor_port, rid, handle, rect, z).is_ok()
}
