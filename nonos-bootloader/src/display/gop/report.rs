// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::init::preferred_mode;
use super::state::{
    get_dimensions, get_stride, is_initialized, FB_FAILURE, FB_FORMAT_BGR, FB_PHYS_MM, FB_PTR,
    FB_SOURCE, NO_FB_BITMASK, NO_FB_BLT_ONLY, NO_FB_NO_GOP,
};
use crate::log::logger::{log_error, log_warn};
use alloc::format;
use core::sync::atomic::Ordering;

pub fn report_gop_mode() {
    if !is_initialized() {
        // Said in words: with no linear framebuffer there is nothing to draw
        // the splash into, and the kernel boots without a boot console.
        let why = match FB_FAILURE.load(Ordering::Relaxed) {
            NO_FB_NO_GOP => "no Graphics Output Protocol on any handle",
            NO_FB_BLT_ONLY => {
                "GOP is PixelBltOnly: no linear framebuffer, the display is unusable after ExitBootServices"
            }
            NO_FB_BITMASK => "GOP offers only a PixelBitMask layout that is not 32 bit RGB or BGR",
            _ => "the firmware's framebuffer does not cover its own mode",
        };
        log_error("gop", &format!("[GOP] no linear framebuffer: {}", why));
        return;
    }
    let (got_w, got_h) = get_dimensions();
    let source = match FB_SOURCE.load(Ordering::Relaxed) {
        1 => "pinned",
        2 => "native (EDID)",
        3 => "firmware current",
        4 => "largest offered",
        _ => "unknown",
    };
    let mm = FB_PHYS_MM.load(Ordering::Relaxed);
    log_warn(
        "gop",
        &format!(
            "[GOP] {}x{} {} pitch={}px fmt={} fb=0x{:x} panel={}x{}mm",
            got_w,
            got_h,
            source,
            get_stride(),
            if FB_FORMAT_BGR.load(Ordering::Relaxed) { "BGRX" } else { "RGBX" },
            FB_PTR.load(Ordering::Relaxed),
            mm & 0xFFFF,
            mm >> 16
        ),
    );
    let Some((want_w, want_h)) = preferred_mode() else {
        return;
    };
    if want_w as u32 != got_w || want_h as u32 != got_h {
        log_error(
            "gop",
            &format!(
                "[GOP] FALLBACK want={}x{} was not offered, scanning out {}x{}",
                want_w, want_h, got_w, got_h
            ),
        );
    }
}
