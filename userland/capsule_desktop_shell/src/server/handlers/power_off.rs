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

//! Power off from the desktop: the menu item and the power key ask, the
//! confirmation is drawn in the trusted band (server/trusted_band.rs), and
//! only the router's "Power Off" sends the request. Every press or key that
//! answers here carries the router's token, which no other sender's message
//! is given (server/runner/drain.rs).

use super::escape_chord::clear_chrome;
use crate::power_off_client::send::{send, Sent};
use crate::render::layout::Rect;
use crate::server::repaint::repaint;
use crate::state::dialog_keys::Choice;
use crate::state::input_source_rule::Routed;
use crate::state::power_off::POWER_OFF_FAILED;
use crate::state::system_key::POWER_OFF_UNAVAILABLE;
use crate::state::{Context, NotifyLevel};

pub(crate) use super::live_prompt::{install_rect as off_rect, later_rect as cancel_rect};

/// Put the confirmation up, over menus and the Launchpad, on Cancel.
pub fn ask(ctx: &mut Context) {
    ctx.trusted.power_off.ask();
    ctx.dialog_focus.reset();
    clear_chrome(ctx);
}

pub fn click(ctx: &mut Context, by: Routed, px: u32, py: u32) -> bool {
    if !ctx.trusted.power_off.showing() {
        return false;
    }
    let off = hit(off_rect(ctx.width, ctx.height), px, py);
    answer(ctx, by, if off { Choice::Act } else { Choice::Leave });
    true
}

/// Power Off sends the request; Cancel, Esc or a press outside closes the
/// confirmation and sends nothing.
pub fn answer(ctx: &mut Context, by: Routed, choice: Choice) {
    ctx.dialog_focus.reset();
    let go = ctx.trusted.power_off.answer(by, choice == Choice::Act);
    repaint(ctx);
    let Some(go) = go else { return };
    let rid = ctx.issue_request_id();
    let said = match send(go, rid) {
        Sent::Missing => POWER_OFF_UNAVAILABLE,
        Sent::Heard(_) => POWER_OFF_FAILED,
    };
    ctx.toasts.push(said, NotifyLevel::Warn, crate::server::toast_clock::now());
    crate::render::sync_toast_layer(ctx);
}

fn hit(r: Rect, px: u32, py: u32) -> bool {
    px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height
}
