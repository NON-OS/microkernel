// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Launchpad page switcher: shared `Tabs`, one a page.

use alloc::string::String;
use alloc::vec::Vec;

use super::grid::dots_band;
use super::view::pages;
use crate::render::page_tabs::{draw, labels, lit, place};
use crate::render::surface::surface;
use crate::state::Context;
use nonos_toolkit::widgets::{Rect, Tabs};

fn with_strip<R>(ctx: &Context, f: impl FnOnce(Tabs, Rect) -> R) -> R {
    let n = pages(ctx);
    let names = if n < 2 { Vec::new() } else { labels(n) };
    let words: Vec<&str> = names.iter().map(String::as_str).collect();
    let tabs = Tabs::new(&words, ctx.launchpad_page);
    f(tabs, place(&tabs, ctx.width, ctx.height, dots_band()))
}

pub(super) fn paint(ctx: &Context) {
    with_strip(ctx, |tabs, at| draw(&mut surface(ctx), tabs, at, &ctx.tile_pointer));
}

pub(crate) fn hit(ctx: &Context, x: u32, y: u32) -> Option<usize> {
    with_strip(ctx, |tabs, at| tabs.hit(at, x as i32, y as i32))
}

pub(crate) fn tabs_lit(ctx: &Context) -> Option<usize> {
    with_strip(ctx, |tabs, at| lit(&tabs, at, &ctx.tile_pointer))
}
