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

//! The dimmer line under a desktop entry's name saying what it is: a
//! folder's item count, or the kind of file, cut to fit the cell.

use crate::render::measure_aa::measure_aa;
use crate::render::palette;
use crate::render::text_aa::text_aa;
use crate::render::ui_font::META_PX;
use crate::state::Context;

const META_FG: u32 = palette::TEXT_DIM;
pub(super) const META_CAP: usize = 16;

// The dimmer second line, saying what the entry holds or is.
pub(super) fn paint_meta(ctx: &Context, item: &crate::vfs_client::Entry, cx: u32, cw: u32, y: u32) {
    let mut buf = [0u8; META_CAP];
    let meta = meta_text(item, &mut buf);
    let x = cx + cw.saturating_sub(measure_aa(meta, META_PX)) / 2;
    text_aa(ctx, x, y, meta, META_FG, META_PX);
}

// A folder says how many entries it holds; a file says what its suffix makes
// it, since the listing carries no size.
pub(super) fn meta_text<'a>(
    item: &'a crate::vfs_client::Entry,
    buf: &'a mut [u8; META_CAP],
) -> &'a str {
    if item.is_dir {
        return items_label(item.children, buf);
    }
    match item.name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() => ext,
        _ => "File",
    }
}

// `12 items`, singular at one.
pub(super) fn items_label(n: u32, buf: &mut [u8; META_CAP]) -> &str {
    let mut digits = [0u8; 10];
    let mut i = digits.len();
    let mut v = n;
    loop {
        i -= 1;
        digits[i] = b'0' + (v % 10) as u8;
        v /= 10;
        if v == 0 {
            break;
        }
    }
    let d = digits.len() - i;
    buf[..d].copy_from_slice(&digits[i..]);
    let tail: &[u8] = if n == 1 { b" item" } else { b" items" };
    buf[d..d + tail.len()].copy_from_slice(tail);
    core::str::from_utf8(&buf[..d + tail.len()]).unwrap_or("")
}
