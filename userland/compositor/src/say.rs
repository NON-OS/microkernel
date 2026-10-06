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

//! One line on the kernel's log for the display the desktop is drawn on.
//! The Terminal's `log DISPLAY` reads it back on a laptop with no serial
//! port, beside the kernel's `[FB]` line for the same framebuffer.

use alloc::format;

use crate::state::Context;

pub fn say(line: &str) {
    let line = format!("[DISPLAY] {}\n", line);
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

/// Which output the desktop went to, the screen's size and the canvas the
/// clients were given for it.
pub fn say_display(ctx: &Context, path: &str) {
    say(&format!(
        "{} {}x{}, canvas {}x{} at scale {}",
        path, ctx.screen.width, ctx.screen.height, ctx.width, ctx.height, ctx.scale
    ));
}
