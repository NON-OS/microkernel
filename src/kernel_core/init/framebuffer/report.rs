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

use crate::sys::serial::Line;

/// Nothing to map: the desktop will have no display on this machine.
pub(super) fn log_refused(why: &[u8]) {
    Line::new().str(b"[FB] not mapped: ").str(why).str(b"; the desktop has no display").end();
}

/// What the desktop scans out through, once mapped.
pub(super) fn log_mapped(w: u32, h: u32, pitch: u32, bgr: bool, wc: bool, scale: u32) {
    let mut l = Line::new();
    l.str(b"[FB] mapped ").dec(w as u64).str(b"x").dec(h as u64).str(b" pitch=").dec(pitch as u64);
    l.str(if bgr { b" BGRX" } else { b" RGBX" });
    l.str(if wc { b" write-combining" } else { b" uncached" });
    l.str(b" scale=").dec(scale as u64).end();
}
