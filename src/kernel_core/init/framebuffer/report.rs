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

//! The lines `log FB` shows for the framebuffer the desktop is drawn on.

use crate::sys::serial::{print, print_dec, println};

/// Nothing to map: the desktop will have no display on this machine.
pub(super) fn log_refused(why: &[u8]) {
    print(b"[FB] not mapped: ");
    print(why);
    println(b"; the desktop has no display");
}

/// What the desktop scans out through, once mapped.
pub(super) fn log_mapped(w: u32, h: u32, pitch: u32, bgr: bool, wc: bool, scale: u32) {
    print(b"[FB] mapped ");
    print_dec(w as u64);
    print(b"x");
    print_dec(h as u64);
    print(b" pitch=");
    print_dec(pitch as u64);
    print(if bgr { b" BGRX" } else { b" RGBX" });
    print(if wc { b" write-combining" } else { b" uncached" });
    print(b" scale=");
    print_dec(scale as u64);
    println(b"");
}
