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

use super::mode::{fb_covers_mode, linear_bgr};
use super::pick;
use super::state::{
    FB_FORMAT_BGR, FB_HEIGHT, FB_INITIALIZED, FB_PTR, FB_SIZE, FB_STRIDE, FB_WIDTH,
};
use core::sync::atomic::Ordering;
use uefi::proto::console::gop::GraphicsOutput;

pub(super) fn latch_current_mode(gop: &mut GraphicsOutput) -> bool {
    let info = gop.current_mode_info();
    // Checked before frame_buffer(), which asserts on a PixelBltOnly mode.
    let bgr = match linear_bgr(&info) {
        Some(v) => v,
        None => return false,
    };
    let (w, ht) = info.resolution();
    // PixelsPerScanLine is pixels by the spec; a value of four times the
    // width or more can only be a byte count (the quirk stride_to_bytes
    // names), so it is brought back to pixels here and the splash, the
    // probe dots and the handoff all agree on one pitch.
    let mut stride = info.stride();
    if w != 0 && stride >= w.saturating_mul(4) && stride % 4 == 0 {
        stride /= 4;
    }
    if w == 0 || ht == 0 || stride < w || w > pick::MAX_DIM as usize || ht > pick::MAX_DIM as usize
    {
        return false;
    }
    let mut fb = gop.frame_buffer();
    let fb_addr = fb.as_mut_ptr() as u64;
    if fb_addr == 0 || !fb_covers_mode(fb.size(), stride, ht) {
        return false;
    }
    FB_PTR.store(fb_addr, Ordering::SeqCst);
    FB_SIZE.store(fb.size() as u64, Ordering::SeqCst);
    FB_WIDTH.store(w as u32, Ordering::SeqCst);
    FB_HEIGHT.store(ht as u32, Ordering::SeqCst);
    FB_STRIDE.store(stride as u32, Ordering::SeqCst);
    FB_FORMAT_BGR.store(bgr, Ordering::SeqCst);
    FB_INITIALIZED.store(true, Ordering::SeqCst);
    true
}
