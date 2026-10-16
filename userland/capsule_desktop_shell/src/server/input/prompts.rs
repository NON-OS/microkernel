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

//! A press taken by what is drawn over the desktop, in this order: the
//! power-off confirmation, the live and delete prompts, the two consents,
//! the toasts' panel, the Launchpad.

use crate::server::handlers::launchpad;
use crate::state::input_source_rule::Routed;
use crate::state::Context;

/// True when one of them took the press.
pub(super) fn consumed(ctx: &mut Context, by: Routed, px: u32, py: u32) -> bool {
    if super::super::handlers::power_off::click(ctx, by, px, py) {
        return true;
    }
    if super::super::handlers::input_menu::click(ctx, by, px, py) {
        return true;
    }
    if super::super::handlers::live_prompt::click(ctx, px, py) {
        return true;
    }
    if super::super::handlers::delete_prompt::click(ctx, px, py) {
        return true;
    }
    if super::super::handlers::consent::click(ctx, px, py) {
        return true;
    }
    if super::super::handlers::pkg_consent::click(ctx, px, py) {
        return true;
    }
    // A press on the toasts' panel puts it away; it is drawn over every
    // window, so the press is not the window's under it.
    if crate::render::toasts::toast_hit(ctx, px, py) {
        ctx.toasts.clear();
        crate::render::sync_toast_layer(ctx);
        return true;
    }
    // The Launchpad, while open, captures every click: a tile launches its app
    // or tool, and anything else dismisses the overlay.
    if ctx.launchpad_ui.launchpad {
        launchpad::click(ctx, px, py);
        return true;
    }
    false
}
