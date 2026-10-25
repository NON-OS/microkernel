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

//! No framebuffer from the loader: why, said once at boot and kept for
//! nonos_display_refusal (state.rs boot_refusal).

use super::state::REFUSAL;
use crate::boot::handoff::types::{refusal, FbRefusal};
use crate::boot::handoff::BootHandoffV1;
use crate::sys::serial::{print, print_dec, print_hex, println};

pub(super) fn no_framebuffer(handoff: &BootHandoffV1) {
    // A handoff claiming a framebuffer and a refusal both was refused before
    // this (security/framebuffer.rs); were one to reach here, it is Unknown.
    let word = handoff.fb.pixel_format;
    let why = refusal(handoff.flags, word).ok().flatten().unwrap_or(FbRefusal::Unknown(word));
    REFUSAL.call_once(|| why.why_word());
    log_no_framebuffer(why);
}

/// The loader handed over no framebuffer, and why, once at boot.
pub(super) fn log_no_framebuffer(why: FbRefusal) {
    print(b"[GOP] no framebuffer handed over");
    print(why.says().as_bytes());
    match why {
        FbRefusal::Bitmask { bpp } => print_dec(bpp as u64),
        FbRefusal::Unknown(word) => print_hex(word as u64),
        _ => {}
    }
    println(if matches!(why, FbRefusal::Bitmask { .. } | FbRefusal::Unknown(_)) {
        b")"
    } else {
        b""
    });
}
