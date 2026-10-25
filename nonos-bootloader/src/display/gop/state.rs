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

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering};

pub static FB_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub static FB_PTR: AtomicU64 = AtomicU64::new(0);
pub static FB_WIDTH: AtomicU32 = AtomicU32::new(0);
pub static FB_HEIGHT: AtomicU32 = AtomicU32::new(0);
pub static FB_STRIDE: AtomicU32 = AtomicU32::new(0);
pub static FB_FORMAT_BGR: AtomicBool = AtomicBool::new(true);
// FrameBufferSize as the firmware reported it for the latched mode.
pub static FB_SIZE: AtomicU64 = AtomicU64::new(0);
// The panel's physical size from its EDID, packed as pick::pack_mm does.
pub static FB_PHYS_MM: AtomicU32 = AtomicU32::new(0);
// Why the mode was chosen (Pick::Source as u8 + 1), or 0 for none.
pub static FB_SOURCE: AtomicU8 = AtomicU8::new(0);
// Why no framebuffer was latched, one of the NO_FB_* values.
pub static FB_FAILURE: AtomicU8 = AtomicU8::new(NO_FB_NO_GOP);
// The bits per pixel of the deepest PixelBitMask mode refused, or 0.
pub static FB_BITMASK_BPP: AtomicU8 = AtomicU8::new(0);

pub use super::refusal_word::{NO_FB_BAD_GEOMETRY, NO_FB_BITMASK, NO_FB_BLT_ONLY, NO_FB_NO_GOP};

#[inline]
pub fn is_initialized() -> bool {
    FB_INITIALIZED.load(Ordering::Relaxed)
}
#[inline]
pub fn get_dimensions() -> (u32, u32) {
    (FB_WIDTH.load(Ordering::Relaxed), FB_HEIGHT.load(Ordering::Relaxed))
}
#[inline]
pub fn get_stride() -> u32 {
    FB_STRIDE.load(Ordering::Relaxed)
}

#[inline]
pub fn convert_color(argb: u32) -> u32 {
    if FB_FORMAT_BGR.load(Ordering::Relaxed) {
        argb
    } else {
        let (a, r, g, b) =
            ((argb >> 24) & 0xFF, (argb >> 16) & 0xFF, (argb >> 8) & 0xFF, argb & 0xFF);
        (a << 24) | (b << 16) | (g << 8) | r
    }
}

/// The linear framebuffer the display module latched and the splash drew to.
#[derive(Clone, Copy, Debug)]
pub struct LatchedFb {
    pub ptr: u64,
    /// FrameBufferSize from the firmware, never less than the frame.
    pub size: u64,
    pub width: u32,
    pub height: u32,
    /// Row pitch in bytes: PixelsPerScanLine times four, not width times four.
    pub stride_bytes: u32,
    pub bgr: bool,
    pub phys_mm: u32,
}

// FB_STRIDE is kept in pixels here, so it is converted to bytes for the
// handoff and the kernel, which both expect a byte stride. The MMIO address
// stays valid across ExitBootServices, so the handoff prefers this proven
// framebuffer over re-querying GOP, which is fragile after mode setup.
pub fn latched_linear_fb() -> Option<LatchedFb> {
    let ptr = FB_PTR.load(Ordering::Acquire);
    let width = FB_WIDTH.load(Ordering::Acquire);
    let height = FB_HEIGHT.load(Ordering::Acquire);
    let stride_px = FB_STRIDE.load(Ordering::Acquire);
    if ptr == 0 || width == 0 || height == 0 || stride_px < width {
        return None;
    }
    let stride_bytes = stride_px.checked_mul(4)?;
    let frame = (stride_bytes as u64).checked_mul(height as u64)?;
    let size = FB_SIZE.load(Ordering::Acquire).max(frame);
    Some(LatchedFb {
        ptr,
        size,
        width,
        height,
        stride_bytes,
        bgr: FB_FORMAT_BGR.load(Ordering::Acquire),
        phys_mm: FB_PHYS_MM.load(Ordering::Acquire),
    })
}

/// Why no framebuffer was latched, as the word the handoff carries it in
/// (refusal_word.rs).
pub fn refusal() -> u32 {
    let code = FB_FAILURE.load(Ordering::Acquire);
    super::refusal_word::refusal_word(code, FB_BITMASK_BPP.load(Ordering::Acquire))
}

pub fn shutdown_for_exit() {
    FB_INITIALIZED.store(false, Ordering::SeqCst);
}
