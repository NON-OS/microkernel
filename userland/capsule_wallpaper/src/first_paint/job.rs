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
//! The built-in picture, painted into the backing: ink first, then the
//! picture scaled over it when it decodes. It runs on a worker thread
//! (worker.rs) and touches nothing but the backing, which the service thread
//! leaves alone until it is done (step.rs). Pure; held in
//! wallpaper_catalog_proofs.

use crate::paint::blit_argb::blit_argb;
use crate::paint::{decode_jpeg, fill_argb};

/// Ink, the desktop's darkest tone, shows if no image decodes.
pub const DEFAULT_ARGB: u32 = 0xFF0A_0B0D;

/// The backing's address and size; `stride` counts bytes.
#[derive(Clone, Copy)]
pub struct Backing {
    pub va: u64,
    pub stride: u32,
    pub width: u32,
    pub height: u32,
}

/// Ink, then `jpeg` over it. True when the picture is on the backing, false
/// when ink alone is.
pub fn paint(b: Backing, jpeg: &[u8]) -> bool {
    fill_argb(b.va, b.stride, b.width, b.height, DEFAULT_ARGB);
    let Some(img) = decode_jpeg(jpeg) else {
        return false;
    };
    blit_argb(b.va, b.stride, b.width, b.height, &img.pixels, img.width, img.height)
}
