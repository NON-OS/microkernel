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

//! Paint the power-off confirmation: the prompts' centred panel, drawn over
//! the toasts, since a toast's words may come from any app, and put in the
//! trusted band while it is up, where no window draws over it.

use super::dialog_kit::{border, button, fill, margin, BORDER, CANCEL_BG, DIM, FG, PANEL};
use super::dialog_text::{line, wrap_at};
use crate::render::ui_font::{line_h, TITLE_PX, UI_PX};
use crate::server::handlers::live_prompt::panel_rect;
use crate::server::handlers::power_off::{cancel_rect, off_rect};
use crate::state::dialog_keys::Choice;
use crate::state::Context;

/// A red, so the button that ends the session does not read as the safe one.
const OFF_BG: u32 = 0xFFA1_2A2A;
const TITLE: &str = "Power off and wipe memory?";
const NOTE: &str = "Every window closes and memory is wiped before the machine turns off. \
Anything not saved is gone.";

pub fn paint_power_off(ctx: &Context) {
    if !ctx.trusted.power_off.showing() {
        return;
    }
    let p = panel_rect(ctx.width, ctx.height);
    fill(ctx, p, PANEL);
    border(ctx, p, BORDER);
    let x = p.x + margin();
    let max_w = p.width.saturating_sub(margin() * 2);
    let mut y = p.y + margin();
    line(ctx, x, y, TITLE, FG, TITLE_PX, max_w);
    y += line_h(TITLE_PX) + line_h(UI_PX) / 3;
    let off = off_rect(ctx.width, ctx.height);
    let mut rest = NOTE;
    while !rest.is_empty() && y + line_h(UI_PX) <= off.y {
        let end = wrap_at(rest, UI_PX, max_w);
        line(ctx, x, y, &rest[..end], DIM, UI_PX, max_w);
        rest = rest[end..].trim_start();
        y += line_h(UI_PX);
    }
    let act = ctx.dialog_focus.focused() == Choice::Act;
    button(ctx, off, OFF_BG, "Power Off", act);
    button(ctx, cancel_rect(ctx.width, ctx.height), CANCEL_BG, "Cancel", !act);
}
