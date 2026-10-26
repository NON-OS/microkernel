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

use crate::state::Context;

pub fn paint_chrome(ctx: &mut Context) {
    let frame_start = crate::frametime::begin();
    let row_words = (ctx.stride / 4) as usize;
    let words = row_words * ctx.height as usize;
    let (desk, chrome) = (ctx.desk.next_va(), ctx.chrome.next_va());
    // Every painter draws on `backing_va`; it points at the off-screen frame
    // for each pass, and at the chrome again after them.
    let mut back = core::mem::take(&mut ctx.back);
    super::super::whole_frame::draw_whole(&mut back, desk, words, row_words, |va| {
        ctx.backing_va = va;
        paint_desk(ctx);
    });
    super::super::whole_frame::draw_whole(&mut back, chrome, words, row_words, |va| {
        ctx.backing_va = va;
        super::paint_over_windows(ctx);
    });
    ctx.back = back;
    super::show::show(ctx);
    ctx.backing_va = ctx.chrome.shown_va();
    crate::frametime::end(frame_start);
}

/// The desktop's icons go on the desk surface, under every window.
fn paint_desk(ctx: &mut Context) {
    super::clear_overlay::clear_overlay(ctx);
    super::super::desktop_icons::paint_desktop_icons(ctx);
}
