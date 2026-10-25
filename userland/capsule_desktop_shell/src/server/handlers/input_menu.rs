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

//! The display input menu: opened from the system menu, drawn in the
//! trusted band, answered only by keys and presses carrying the router's token.

use super::escape_chord::clear_chrome;
use super::input_menu_geometry::row_at;
use crate::compositor_client::input_sources::{offered, switch};
use crate::server::repaint::repaint;
use crate::state::input_menu::{Switch, NONE_OFFERED, REFUSED, SWITCHED};
use crate::state::input_offers::read;
use crate::state::input_source_rule::Routed;
use crate::state::{Context, NotifyLevel};

/// Ask the displays what they offer and open the menu on it.
pub fn open(ctx: &mut Context) {
    if ctx.trusted.power_off.showing() {
        return;
    }
    let rid = ctx.issue_request_id();
    let got = offered(ctx.ports.compositor_port, rid).map(|a| read(&a));
    if !got.is_some_and(|(o, n)| ctx.trusted.input_menu.open(&o[..n])) {
        return say(ctx, NONE_OFFERED, NotifyLevel::Info);
    }
    clear_chrome(ctx);
    repaint(ctx);
}

pub fn key(ctx: &mut Context, by: Routed, code: u32) -> bool {
    if !ctx.trusted.input_menu.showing() {
        return false;
    }
    let go = ctx.trusted.input_menu.key(by, code);
    finish(ctx, go);
    true
}

pub fn click(ctx: &mut Context, by: Routed, px: u32, py: u32) -> bool {
    if !ctx.trusted.input_menu.showing() {
        return false;
    }
    let row = row_at(&ctx.trusted.input_menu, ctx.width, ctx.height, px, py);
    let go = ctx.trusted.input_menu.choose(by, row);
    finish(ctx, go);
    true
}

fn finish(ctx: &mut Context, go: Option<Switch>) {
    repaint(ctx);
    let Some(go) = go else { return };
    let rid = ctx.issue_request_id();
    match switch(ctx.ports.compositor_port, rid, go) {
        Ok(0) => say(ctx, SWITCHED, NotifyLevel::Info),
        _ => say(ctx, REFUSED, NotifyLevel::Warn),
    }
}

fn say(ctx: &mut Context, text: &[u8], level: NotifyLevel) {
    ctx.toasts.push(text, level, crate::server::toast_clock::now());
    crate::render::sync_toast_layer(ctx);
}
