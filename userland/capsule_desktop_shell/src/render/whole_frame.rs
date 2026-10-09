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

//! A shell frame reaches its surface whole.
//!
//! The compositor reads the shell's surfaces whenever it composites, on
//! another CPU, while the shell paints. Every chrome paint cleared the whole
//! surface to transparent and built the menubar, the dock, menus and dialogs
//! back up in place, and the desk the same with its icons, so a composite in
//! between showed the windows and the wallpaper where the dock or an icon
//! was. A composite runs for every cursor move, and the shell repaints on the
//! clock each second, on dock and menu hover and on every step of an icon
//! drag, so the dock and the icons blinked in squares around the pointer.
//! Each frame is drawn off screen now and copied over its surface after, so
//! the compositor only ever reads a whole frame, old or new. Only rows that
//! changed are written: a clock tick touches the menubar's rows alone.

use alloc::vec::Vec;

/// Draw one frame of `words` pixels with `draw`, which is handed the address
/// to draw at, and leave it on the surface at `target`. `back` is kept by
/// the caller and reused from frame to frame. When the heap cannot hold a
/// frame, `draw` draws on the surface itself, as before.
pub fn draw_whole(
    back: &mut Vec<u32>,
    target: u64,
    words: usize,
    row_words: usize,
    draw: impl FnOnce(u64),
) {
    if back.len() != words {
        back.clear();
        if row_words == 0 || back.try_reserve_exact(words).is_err() {
            draw(target);
            return;
        }
        back.resize(words, 0);
    }
    draw(back.as_mut_ptr() as u64);
    // SAFETY: `target` is the caller's mapped surface of `words` pixels, the
    // same span every painter writes, and `back` is a separate allocation.
    let surface = unsafe { core::slice::from_raw_parts_mut(target as *mut u32, words) };
    for (dst, src) in surface.chunks_mut(row_words).zip(back.chunks(row_words)) {
        if dst != src {
            dst.copy_from_slice(src);
        }
    }
}
