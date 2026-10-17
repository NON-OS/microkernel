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

//! An input frame from the router: checked, then the secure attention
//! chord first, keys, the wheel, a drag's release, motion and presses.

use crate::protocol::{read_i32, read_u16, read_u32};
use crate::server::handlers::{self, launchpad};
use crate::state::input_source_rule::Routed;
use crate::state::Context;
use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_WHEEL};

pub fn handle(ctx: &mut Context, by: Routed, buf: &[u8]) -> bool {
    if buf.len() < 40 || read_u32(buf, 0) != Some(0x4E49_4E50) {
        return false;
    }
    if read_u16(buf, 4) != Some(1) {
        return true;
    }
    // The router's notice is no event: said, and nothing routed.
    if crate::state::router_notice::is_notice(buf) {
        handlers::bond_notice::say(ctx);
        return true;
    }
    let Some(kind) = read_u16(buf, 8) else {
        return true;
    };
    // Ctrl+Alt+Delete and the hidden screen come before anything else the shell does.
    if let Some(done) = super::super::handlers::attention::filter(ctx, kind, buf) {
        return done;
    }
    // A key press while an inline rename is active edits the name; keys carry no
    // pointer position, so this is handled before the x/y checks below.
    if kind == INPUT_KIND_KEY_DOWN {
        return super::key::key(ctx, by, buf, kind);
    }
    // The wheel reaches the shell only through its pointer grab, which the
    // Launchpad holds while open; it was dropped here, so the pages could
    // only be turned by their dots.
    if kind == INPUT_KIND_WHEEL {
        if ctx.launchpad_ui.launchpad {
            launchpad::wheel(ctx, read_i32(buf, 28).unwrap_or(0));
        }
        return true;
    }
    super::pointer::pointer(ctx, by, buf, kind)
}
