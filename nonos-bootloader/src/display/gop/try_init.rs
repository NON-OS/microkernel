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

use super::edid::read_edid;
use super::init::preferred_mode;
use super::latch::latch_current_mode;
use super::mode::linear_bgr;
use super::offered::offered_modes;
use super::pick::{self, Choice};
use super::state::{FB_FAILURE, FB_PHYS_MM, FB_SOURCE};
use core::sync::atomic::Ordering;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::BootServices;

pub(super) fn try_init(bs: &BootServices, h: Handle) -> bool {
    let edid = read_edid(bs, h);
    let mut gop = match bs.open_protocol_exclusive::<GraphicsOutput>(h) {
        Ok(g) => g,
        Err(_) => return false,
    };
    let modes = offered_modes(&gop);
    let current_info = gop.current_mode_info();
    let current = linear_bgr(&current_info).map(|_| {
        let (w, ht) = current_info.resolution();
        (w as u32, ht as u32)
    });
    let native = edid.map(|e| (e.width, e.height));
    let pinned = preferred_mode().map(|(w, ht)| (w as u32, ht as u32));
    let phys_mm = edid.map_or(0, |e| pick::pack_mm(e.width_mm, e.height_mm));
    FB_PHYS_MM.store(phys_mm, Ordering::SeqCst);
    match pick::choose(&modes.offered, current, native, pinned) {
        Some((Choice::Set(idx), source)) => {
            if let Ok(mode) = gop.query_mode(idx) {
                if gop.set_mode(&mode).is_ok() && latch_current_mode(&mut gop) {
                    FB_SOURCE.store(source as u8 + 1, Ordering::SeqCst);
                    return true;
                }
            }
        }
        Some((Choice::Keep, source)) => {
            if latch_current_mode(&mut gop) {
                FB_SOURCE.store(source as u8 + 1, Ordering::SeqCst);
                return true;
            }
        }
        None => {}
    }
    // The chosen mode would not set or latch; whatever the firmware left
    // set is still better than no splash.
    if latch_current_mode(&mut gop) {
        FB_SOURCE.store(pick::Source::Current as u8 + 1, Ordering::SeqCst);
        return true;
    }
    FB_FAILURE.store(modes.failure(), Ordering::SeqCst);
    false
}
