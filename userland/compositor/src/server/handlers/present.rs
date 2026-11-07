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

//! A swapchain client shows another of its buffers (nonos_surface::Present).
//! The layer keeps its place and size; only the changed part is repainted.
//! The buffer it showed until now is held until the next frame is composed,
//! then its release fence is signalled, so the client never draws in a
//! buffer the compositor may still read.

use crate::protocol::{Request, E_BUSY, E_INVAL, E_PERM};
use crate::requests::parts::Parts;
use crate::requests::present::Shown;
use crate::server::respond;
use crate::state::damage::Rect;
use crate::state::hidden::shown_on;
use crate::state::present_rule::{present, present_on, Swapped};
use crate::state::Context;
use crate::state::SceneTable;

use super::answers::answers_for;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some(shown) = crate::requests::present::present(body) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    take(ctx, sender, req, tx, (shown, None), |scene, changed| {
        present(scene, sender, shown.z, shown.handle, changed)
    })
}

/// OP_PRESENT_LAYER: the same, for the sender's layer showing the named
/// handle, so a client with a window per layer in one band names each.
pub fn handle_layer(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some((shown, on)) = crate::requests::present::present_layer(body) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    take(ctx, sender, req, tx, (shown, None), |scene, changed| {
        present_on(scene, sender, shown.z, on, shown.handle, changed)
    })
}

/// OP_PRESENT_LAYER_PARTS: OP_PRESENT_LAYER repainting only the boxes its
/// parts block names inside its changed box.
pub fn handle_layer_parts(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some((shown, on, parts)) = crate::requests::present::present_layer_parts(body) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    take(ctx, sender, req, tx, (shown, Some(parts)), |scene, changed| {
        present_on(scene, sender, shown.z, on, shown.handle, changed)
    })
}

fn take(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    tx: &mut [u8],
    (shown, parts): (Shown, Option<Parts>),
    rule: impl FnOnce(&mut SceneTable, Rect) -> Option<Swapped>,
) -> Result<(), &'static str> {
    // Room first: a buffer taken off screen is never let go before its time.
    if !ctx.retiring.has_room() {
        return respond::status(sender, req, E_BUSY, tx);
    }
    // Only a buffer the sender answers for takes the layer's place.
    if !answers_for(ctx, sender, shown.handle) {
        return respond::status(sender, req, E_PERM, tx);
    }
    let a = shown.area;
    let changed = Rect { x: a.x, y: a.y, width: a.width, height: a.height };
    let Some(swapped) = rule(&mut ctx.scene, changed) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    if let Some(old) = swapped.old {
        ctx.retiring.hold(old);
    }
    ctx.direct.presented(shown.handle, shown.opaque);
    ctx.scene.set_opaque(sender, shown.handle, shown.opaque);
    // A changed part wholly under an opaque layer over this one shows
    // nothing and is left (state/hidden.rs).
    let on = (sender, shown.z, shown.handle);
    match parts {
        None => repaint(ctx, on, swapped.repaint),
        Some(parts) => {
            for a in parts.iter() {
                let part = Rect { x: a.x, y: a.y, width: a.width, height: a.height };
                repaint(ctx, on, swapped.repaint_of(part));
            }
        }
    }
    respond::status(sender, req, 0, tx)
}

/// `r` repainted unless it is wholly under an opaque layer over the one
/// `on` names.
fn repaint(ctx: &mut Context, on: (u32, u32, u64), r: Option<Rect>) {
    if let Some(r) = r.and_then(|r| shown_on(&ctx.scene, on, r)) {
        ctx.damage.accumulate(r);
    }
}
