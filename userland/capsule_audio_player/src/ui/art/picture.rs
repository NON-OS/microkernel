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

//! The playing track's own cover, from its tags, drawn where the generated
//! artwork would be. Only the playing track's is decoded, once, at about the
//! size it is shown: a library's every cover decoded on a Celeron would cost
//! more than the list is worth, and the rows keep their generated art.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use spin::Mutex;

use crate::ui::geometry::Rect;

/// A decoded picture, ARGB rows.
pub struct Picture {
    pub w: u32,
    pub h: u32,
    pub px: Vec<u32>,
}

/// The track whose cover is held, by path and by title (the rail names the
/// track by its title, the bar and Now Playing by its path), and the cover.
static CURRENT: Mutex<Option<(String, String, Picture)>> = Mutex::new(None);

/// Hold `picture` as the cover of the track at `path` called `title`, or
/// drop the one held when the track has none.
pub fn set_current(path: &str, title: &str, picture: Option<Picture>) {
    *CURRENT.lock() = picture.map(|p| (String::from(path), String::from(title), p));
}

/// Draw the held cover into `r` if `id` names its track. True when drawn.
pub fn draw_if_current(fb: &mut PaintBuffer, r: Rect, id: &str) -> bool {
    let held = CURRENT.lock();
    let Some((path, title, p)) = held.as_ref() else { return false };
    if id.is_empty() || (id != path && id != title) {
        return false;
    }
    fill_cropped(fb, r, p);
    true
}

/// `p` scaled to fill `r`, its middle kept when the shapes differ, as an
/// album cover is shown in every player.
fn fill_cropped(fb: &mut PaintBuffer, r: Rect, p: &Picture) {
    if p.w == 0 || p.h == 0 || r.w <= 0 || r.h <= 0 {
        return;
    }
    let (rw, rh) = (r.w as u64, r.h as u64);
    // The source square (or rectangle) that maps onto r at one scale.
    let (sw, sh) = if u64::from(p.w) * rh > u64::from(p.h) * rw {
        ((u64::from(p.h) * rw / rh).max(1), u64::from(p.h))
    } else {
        (u64::from(p.w), (u64::from(p.w) * rh / rw).max(1))
    };
    let (sx0, sy0) = ((u64::from(p.w) - sw) / 2, (u64::from(p.h) - sh) / 2);
    let x0 = r.x.max(0);
    let x1 = r.right().min(fb.width as i32);
    let y0 = r.y.max(0);
    let y1 = r.bottom().min(fb.height as i32);
    for y in y0..y1 {
        let sy = sy0 + (y - r.y) as u64 * sh / rh;
        let row = (sy.min(u64::from(p.h) - 1) * u64::from(p.w)) as usize;
        let out = y as usize * fb.stride_words as usize;
        for x in x0..x1 {
            let sx = sx0 + (x - r.x) as u64 * sw / rw;
            let c = p.px[row + sx.min(u64::from(p.w) - 1) as usize];
            fb.pixels[out + x as usize] = 0xFF00_0000 | (c & 0x00FF_FFFF);
        }
    }
}
