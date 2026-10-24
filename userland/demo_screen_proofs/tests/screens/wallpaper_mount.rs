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

//! The wallpaper capsule's decode and cover-scale, mounted as shipped, over
//! the catalog's first wallpaper.

#[path = "../../../capsule_wallpaper/src/paint/blit_argb.rs"]
mod blit_argb;
#[path = "../../../capsule_wallpaper/src/paint/decode_jpeg.rs"]
mod decode_jpeg;

/// The wallpaper as the capsule paints it on a `w` x `h` canvas.
pub fn wallpaper(w: u32, h: u32) -> Vec<u32> {
    let file =
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../nonos-data/wallpapers/field-focus-1.jpg");
    let img =
        decode_jpeg::decode_jpeg(&std::fs::read(file).expect("the wallpaper")).expect("decoded");
    let mut px = vec![0u32; (w * h) as usize];
    assert!(blit_argb::blit_argb(
        px.as_mut_ptr() as u64,
        w * 4,
        w,
        h,
        &img.pixels,
        img.width,
        img.height
    ));
    px
}
