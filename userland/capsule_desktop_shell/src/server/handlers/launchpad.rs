// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Opening the Launchpad from the dock, and acting on a click while it is open:
//! a tile launches its app, tool or capsule-store app, a click on the search
//! pill or the page dots belongs to that chrome, and a click on empty space
//! dismisses the overlay. A store app is absent from the kernel's compile-time
//! spawn table, so it goes to the installer rather than the dock's spawn path.

use super::launchpad_launch::launch;
use crate::render::launchpad::{
    dots_hit, hit_target, page_slice, pages as launchpad_pages, rebuild, search_hit,
};
use crate::server::repaint::repaint;
use crate::state::Context;

/// KEY_DOWN kind bit, grabbed for the overlay's lifetime so keys typed into the
/// search field reach the shell rather than whatever window sits behind it.
pub fn open(ctx: &mut Context) {
    ctx.launchpad_ui.launchpad = true;
    ctx.launchpad_ui.launchpad_query.clear();
    ctx.launchpad_ui.launchpad_page = 0;
    ctx.launchpad_ui.launchpad_wheel.reset();
    rebuild(ctx);
    crate::server::grabs::sync(ctx);
    repaint(ctx);
}

pub fn close(ctx: &mut Context) {
    ctx.launchpad_ui.launchpad = false;
    crate::server::grabs::sync(ctx);
    ctx.launchpad_ui.launchpad_query.clear();
    ctx.launchpad_ui.launchpad_page = 0;
    ctx.launchpad_ui.launchpad_wheel.reset();
    repaint(ctx);
}

/// The wheel over the open Launchpad turns its pages (state::launchpad_wheel).
/// Its pointer grab brings every wheel event here, wherever the pointer is.
pub fn wheel(ctx: &mut Context, delta_y: i32) {
    let pages = launchpad_pages(ctx);
    let page =
        ctx.launchpad_ui.launchpad_wheel.turn(ctx.launchpad_ui.launchpad_page, pages, delta_y);
    if page != ctx.launchpad_ui.launchpad_page {
        ctx.launchpad_ui.launchpad_page = page;
        repaint(ctx);
    }
}

pub fn click(ctx: &mut Context, px: u32, py: u32) {
    if let Some(page) = dots_hit(ctx, px, py) {
        ctx.launchpad_ui.launchpad_page = page;
        repaint(ctx);
        return;
    }
    if search_hit(ctx, px, py) {
        return;
    }
    match hit_target(ctx, px, py) {
        Some(t) => launch(ctx, t),
        None => close(ctx),
    }
}

/// Enter runs the first entry the current filter left standing, so a search can
/// be completed without reaching for the pointer.
pub fn launch_first(ctx: &mut Context) {
    let first = page_slice(ctx).first().copied();
    if let Some(t) = first {
        launch(ctx, t);
    }
}
