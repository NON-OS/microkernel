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

//! The secure attention chord as the shell answers it (`state/attention.rs`):
//! the chord opens the system's prompt, or leaves the lock; while the lock
//! is up every other key and click is swallowed; the prompt's buttons lock
//! the screen or go back. What is up is drawn in the chrome, which the
//! repaint puts in the compositor's trusted band (`server/trusted_band.rs`).

use nonos_libc::{INPUT_KIND_BUTTON_DOWN, INPUT_KIND_TOUCH};

use super::escape_chord::{clear_chrome, is_secure_attention};
use crate::protocol::{read_i32, read_u16, read_u32};
use crate::render::layout::Rect;
use crate::server::repaint::repaint;
use crate::state::dialog_keys::Choice;
use crate::state::Context;

pub(crate) use super::live_prompt::{
    install_rect as lock_rect, later_rect as back_rect, panel_rect,
};

/// Some(true) when the event was the chord's, the lock's or the prompt's
/// click; None to route it as before.
pub fn filter(ctx: &mut Context, kind: u16, buf: &[u8]) -> Option<bool> {
    let (code, flags) = (read_u32(buf, 12).unwrap_or(0), read_u16(buf, 10).unwrap_or(0));
    if is_secure_attention(kind, code, flags) {
        clear_chrome(ctx);
        ctx.trusted.attention = ctx.trusted.attention.chord();
        ctx.dialog_focus.reset();
        repaint(ctx);
        return Some(true);
    }
    if ctx.trusted.attention.locked() {
        return Some(true);
    }
    let press = kind == INPUT_KIND_BUTTON_DOWN || kind == INPUT_KIND_TOUCH;
    if !ctx.trusted.attention.prompt() || !press {
        return None;
    }
    let (x, y) = (read_i32(buf, 16).unwrap_or(-1), read_i32(buf, 20).unwrap_or(-1));
    let lock = x >= 0 && y >= 0 && hit(lock_rect(ctx.width, ctx.height), x as u32, y as u32);
    answer(ctx, if lock { Choice::Act } else { Choice::Leave });
    Some(true)
}

/// Lock the screen, or go back; a press outside the buttons goes back.
pub fn answer(ctx: &mut Context, choice: Choice) {
    ctx.dialog_focus.reset();
    ctx.trusted.attention = ctx.trusted.attention.answer(choice == Choice::Act);
    repaint(ctx);
}

fn hit(r: Rect, px: u32, py: u32) -> bool {
    px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height
}
