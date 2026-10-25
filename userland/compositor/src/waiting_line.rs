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

//! The line said while waiting for a display (refusal_note.rs says when).

use alloc::format;
use alloc::string::String;

use nonos_libc::nonos_display_refusal;

use crate::refusal_note::{reason, LOG_AFTER_MS};

/// `gop_why` is run_gop's own reason, said when the kernel has a framebuffer
/// and so no refusal to name; `later` is the line after the bound.
pub fn waiting_line(gop_why: &str, later: bool) -> String {
    let mut word = 0u32;
    let why = if nonos_display_refusal(0, &mut word as *mut u32) < 0 {
        match word & 0xFF {
            2 => format!("no framebuffer: {} ({} bits a pixel)", reason(word), (word >> 8) & 0xFF),
            _ => format!("no framebuffer: {}", reason(word)),
        }
    } else {
        String::from(gop_why)
    };
    if later {
        let secs = LOG_AFTER_MS / 1000;
        format!("still no display {} s on ({}); waiting for a display driver", secs, why)
    } else {
        format!("no display yet ({}); waiting for a display driver", why)
    }
}
