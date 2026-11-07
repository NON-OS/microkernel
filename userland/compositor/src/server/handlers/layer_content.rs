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

//! OP_LAYER_CONTENT: the sender's layer draws a surface over its own, or no
//! longer does. The surface it drew until now is held until the next frame
//! is composed, then its release fence is signalled, as a present's old
//! buffer is (present.rs), so its owner never draws in it while it is read.

use crate::protocol::{Request, E_BUSY, E_INVAL, E_PERM};
use crate::requests::content::{layer_content, layer_content_parts, ContentSet};
use crate::requests::parts::Parts;
use crate::server::respond;

use super::answers::answers_for;
use crate::state::content_rule::repaint;
use crate::state::damage::Rect;
use crate::state::hidden::hidden;
use crate::state::scene::Content;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some(set) = layer_content(body) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    take(ctx, sender, req, tx, (set, Parts::one(set.area)))
}

/// OP_LAYER_CONTENT_PARTS: OP_LAYER_CONTENT repainting only the boxes its
/// parts block names inside the box that changed.
pub fn handle_parts(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some((set, parts)) = layer_content_parts(body) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    take(ctx, sender, req, tx, (set, parts))
}

fn take(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    tx: &mut [u8],
    (set, parts): (ContentSet, Parts),
) -> Result<(), &'static str> {
    // Room first: a surface taken off screen is never let go before its time.
    if !ctx.retiring.has_room() {
        return respond::status(sender, req, E_BUSY, tx);
    }
    if set.handle != 0 && !answers_for(ctx, sender, set.handle) {
        return respond::status(sender, req, E_PERM, tx);
    }
    let v = set.view;
    let view = (v.x, v.y, v.width, v.height);
    let now = Content { handle: set.handle, x: set.x, y: set.y, opaque: set.opaque, view };
    let Some(was) = ctx.scene.set_content(sender, set.on, now) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    let old = was.content.handle;
    if old != 0 && old != set.handle {
        ctx.retiring.hold(old);
    }
    let now = if set.handle == 0 { Content::default() } else { now };
    for a in parts.iter() {
        let changed = Rect { x: a.x, y: a.y, width: a.width, height: a.height };
        // A part wholly under an opaque layer over this one shows nothing
        // and is left (state/hidden.rs).
        let shown = |r| !hidden(ctx.scene.layers(), &was, r);
        if let Some(r) = repaint(&was, now, changed).filter(|&r| shown(r)) {
            ctx.damage.accumulate(r);
        }
    }
    respond::status(sender, req, 0, tx)
}
