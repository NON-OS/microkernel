// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ctrl+V in the shell's own text fields. The rename box and the Launchpad
//! search took typed keys only, and a Ctrl+V typed a "v" into them. The text
//! comes from the clipboard service through the same client every app uses.

use alloc::string::String;

use nonos_app_skeleton::clipboard_paste;
use nonos_app_skeleton::log_line::{say as log, Line};

use crate::state::paste_line::paste_line;
use crate::state::{Context, NotifyLevel};

/// The ASCII letter V, either case, as a key code arrives with Ctrl held.
pub fn is_paste_key(code: u32) -> bool {
    code == 0x56 || code == 0x76
}

/// Paste the clipboard's first line into `field` (at most `max_bytes`).
/// Returns whether the field changed; says so on screen when the clipboard
/// service cannot be reached, since the paste would otherwise look ignored.
pub fn paste_into(ctx: &mut Context, field: &mut String, max_bytes: usize) -> bool {
    let mut scratch = [0u8; 256];
    match clipboard_paste(&mut scratch) {
        Ok(n) => paste_line(field, &scratch[..n], max_bytes),
        Err(why) => {
            ctx.toasts.push(
                b"clipboard unavailable",
                NotifyLevel::Warn,
                crate::server::toast_clock::now(),
            );
            let line = Line::new(b"SHELL");
            let _ = log(&line.text(b"paste: clipboard unavailable: ").text(why.as_bytes()));
            false
        }
    }
}
