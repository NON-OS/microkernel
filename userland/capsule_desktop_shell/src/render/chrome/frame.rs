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

//! The shell's whole paint: the desk surface under the windows, then the
//! chrome over them, each drawn off-screen whole and copied over its
//! surface, the painters pointed at each frame in turn.

use crate::render::changed_rows::rows_apart;
use crate::state::ctx::flip::Flip;
use crate::state::Context;

use super::sent::Painted;
use super::show::Changed;

pub fn paint_chrome(ctx: &mut Context) -> Painted {
    paint_chrome_in(ctx, Changed::Whole)
}

/// The same, for a frame that changed only `changed` of the screen: the
/// whole frame is drawn, and only that part is composed again (show.rs).
/// A surface whose part frame came out the same as the one on screen is
/// not shown again (render/whole_frame.rs). What became of each surface's
/// frame, and so whether the caller owes a damage commit (sent.rs).
pub fn paint_chrome_in(ctx: &mut Context, changed: Changed) -> Painted {
    let frame_start = crate::frametime::begin();
    let row_words = (ctx.stride / 4) as usize;
    let words = row_words * ctx.height as usize;
    let (desk, chrome) = (ctx.desk.next_va(), ctx.chrome.next_va());
    // A whole frame is held to the one on screen as a part frame is, so a
    // surface it did not change is not shown again (render/changed_rows.rs).
    let (desk_shown, chrome_shown) = (Some(ctx.desk.shown_va()), Some(ctx.chrome.shown_va()));
    // Every painter draws on `backing_va`; it points at the off-screen frame
    // for each pass, and at the chrome again after them.
    let mut back = core::mem::take(&mut ctx.back);
    let desk_new = super::super::whole_frame::draw_whole(
        &mut back,
        desk,
        desk_shown,
        words,
        row_words,
        |va| {
            ctx.backing_va = va;
            paint_desk(ctx);
        },
    );
    let chrome_new = super::super::whole_frame::draw_whole(
        &mut back,
        chrome,
        chrome_shown,
        words,
        row_words,
        |va| {
            ctx.backing_va = va;
            super::paint_over_windows(ctx);
        },
    );
    ctx.back = back;
    let rows = |new: bool, f: Flip| {
        let on_two = new && f.flips();
        on_two.then(|| rows_apart(f.next_va(), f.shown_va(), words, row_words)).flatten()
    };
    let spans = (rows(desk_new, ctx.desk), rows(chrome_new, ctx.chrome));
    let painted = super::show::show(ctx, changed, (desk_new, chrome_new), spans);
    ctx.backing_va = ctx.chrome.shown_va();
    crate::frametime::end(frame_start);
    painted
}

/// The desktop's icons go on the desk surface, under every window.
fn paint_desk(ctx: &mut Context) {
    super::clear_overlay::clear_overlay(ctx);
    super::super::desktop_icons::paint_desktop_icons(ctx);
}
