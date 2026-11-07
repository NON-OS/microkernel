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

//! Redraw the chrome and immediately present it. Input handlers must use this
//! rather than calling paint alone: painting only updates the backing buffer,
//! so without the damage commit the compositor never shows the change until the
//! next clock tick, which leaves menus half-drawn or ghosted behind the cursor.
//! A frame the compositor took was damaged by the request that took it, so
//! only what that request did not damage is committed (render/chrome/sent.rs);
//! committed after it, the same part, or the whole screen, was composed a
//! second time.

use crate::compositor_client::push_damage_commit;
use crate::render::layout::{dock_area_rect, Rect};
use crate::render::toasts::place::toast_rect;
use crate::render::{menubar_menu, menubar_rect, paint_chrome, paint_chrome_in, Changed};
use crate::state::taskbar::{dropdown_parts, focus_parts, part_frame, Area, Parts};
use crate::state::Context;

pub fn repaint(ctx: &mut Context) {
    let whole = (0, 0, ctx.width, ctx.height);
    for &(x, y, w, h) in paint_chrome(ctx).owed().of(&[whole]) {
        let rid = ctx.issue_request_id();
        let _ = push_damage_commit(ctx.ports.compositor_port, rid, x, y, w, h);
    }
    ctx.taskbar.drawn = ctx.taskbar.visible;
    super::trusted_band::sync(ctx);
}

/// The same, for a change that stays inside `r` (the row lit in an open
/// menu): the whole chrome is drawn, and only `r` is composed again
/// (render/chrome/show.rs), with the toasts' area when toasts not yet drawn
/// go on this frame. A dock shown or hidden, or the lock screen, is shown
/// whole.
pub fn repaint_part(ctx: &mut Context, r: Rect) {
    let Some(parts) = owed_parts(ctx, &[area(r)], dock_owed(ctx)) else {
        return repaint(ctx);
    };
    show_parts(ctx, parts);
    super::trusted_band::sync(ctx);
}

/// The focused app changed (the window manager's Focused notice): the menu
/// bar's first title and the dock's mark name it, so the bar and the dock's
/// area are composed again and nothing between them
/// (state/taskbar/focus_parts.rs). The whole chrome is drawn as ever; the
/// frame is presented with the bar named, and the dock's area is committed
/// after, from the frame now on screen, with the toasts' area when toasts
/// not yet drawn go on it. A frame the same as the one on screen is not
/// shown, and nothing is committed.
pub fn repaint_focus(ctx: &mut Context) {
    let bar = area(menubar_rect(ctx.width));
    let dock = area(dock_area_rect(ctx.width, ctx.height));
    let menu_open = ctx.menubar.open.is_some();
    let whole_owed = dock_owed(ctx) || ctx.lock.screen.covers();
    let Some(strips) = focus_parts(bar, dock, whole_owed, menu_open) else {
        return repaint(ctx);
    };
    let Some(parts) = owed_parts(ctx, &strips, false) else {
        return repaint(ctx);
    };
    show_parts(ctx, parts);
    super::trusted_band::sync(ctx);
}

/// A change of the menu bar's drop-down (opened, closed, or gone to another
/// title): the bar and the drop-down where it was, `was`, its reach before
/// the change, and where it is now (state/taskbar/dropdown_parts.rs), with
/// the toasts' area when toasts not yet drawn go on this frame. A dock shown
/// or hidden, the lock screen, or more parts than a frame names, is shown
/// whole.
pub fn repaint_dropdown(ctx: &mut Context, was: Option<Rect>) {
    let bar = area(menubar_rect(ctx.width));
    let now = menubar_menu::reach(ctx).map(area);
    let (parts, n) = dropdown_parts(bar, was.map(area), now);
    let Some(parts) = owed_parts(ctx, &parts[..n], dock_owed(ctx)) else {
        return repaint(ctx);
    };
    show_parts(ctx, parts);
    super::trusted_band::sync(ctx);
}

/// `changed` and what else this frame owes the screen in part
/// (state/taskbar/part_frame.rs); None when it is owed whole. `dock_owed`:
/// the dock shown or hidden since it was drawn, and this frame not the
/// dock's own.
pub(super) fn owed_parts(ctx: &Context, changed: &[Area], dock_owed: bool) -> Option<Parts> {
    let new_toasts = ctx.toast_view.toasts_drawn != ctx.toasts.generation();
    let toasts = new_toasts.then(|| area(toast_rect(ctx.width, ctx.height)));
    part_frame(changed, toasts, dock_owed, ctx.lock.screen.covers())
}

/// The frame drawn and presented with its first part named, and the parts
/// that present did not damage committed after it: the rest when it was
/// presented, none when it was submitted whole or was the same as the one on
/// screen, every one when the compositor took neither.
pub(super) fn show_parts(ctx: &mut Context, parts: Parts) {
    let (x, y, w, h) = parts.first();
    let painted = paint_chrome_in(ctx, Changed::Part(x, y, w, h));
    for &(x, y, w, h) in painted.owed().of(parts.as_slice()) {
        let rid = ctx.issue_request_id();
        let _ = push_damage_commit(ctx.ports.compositor_port, rid, x, y, w, h);
    }
}

fn dock_owed(ctx: &Context) -> bool {
    ctx.taskbar.drawn != ctx.taskbar.visible
}

fn area(r: Rect) -> Area {
    (r.x, r.y, r.width, r.height)
}
