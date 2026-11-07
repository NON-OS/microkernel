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
//! Each frame is drawn off screen now and copied over a backing after. The
//! copy goes row by row, so a composite during it would still read part of a
//! frame; the backing copied to is the one the compositor is not reading,
//! shown once it is whole (state/ctx/flip.rs). Only rows that changed are
//! written: a clock tick touches the menubar's rows alone.
//!
//! A frame that changed a part only (the clock's tick, the dock, a menu row)
//! and came out the same as the frame on screen is not copied or shown at
//! all: the clock shows minutes, and its tick redrew and presented both
//! surfaces every second, so an idle desktop composed the menu bar about
//! three times a second for no change. A whole frame is always shown.

use alloc::vec::Vec;

/// Draw one frame of `words` pixels with `draw`, which is handed the address
/// to draw at, and leave it on the surface at `target`. `back` is kept by
/// the caller and reused from frame to frame. When the heap cannot hold a
/// frame, `draw` draws on the surface itself, as before. `shown` is the
/// backing on screen, given for a frame that may be left unshown when it is
/// the same; None for a frame that is shown whatever it holds.
///
/// True when the frame is to be shown: drawn on the surface, or differing
/// from the one on screen. False only for a frame the same as `shown`,
/// which leaves `target` as it was.
pub fn draw_whole(
    back: &mut Vec<u32>,
    target: u64,
    shown: Option<u64>,
    words: usize,
    row_words: usize,
    draw: impl FnOnce(u64),
) -> bool {
    if back.len() != words {
        back.clear();
        if row_words == 0 || back.try_reserve_exact(words).is_err() {
            draw(target);
            return true;
        }
        back.resize(words, 0);
    }
    draw(back.as_mut_ptr() as u64);
    let apart = shown.filter(|&va| va != target);
    if let Some(va) = apart {
        // SAFETY: `va` is the caller's other mapped backing of the same
        // `words` pixels, which nothing writes while the shell paints (the
        // compositor maps it read only), and it is not `target`, so no
        // mutable view of it exists here.
        let on_screen = unsafe { core::slice::from_raw_parts(va as *const u32, words) };
        if on_screen == back.as_slice() {
            return false;
        }
    }
    // SAFETY: `target` is the caller's mapped surface of `words` pixels, the
    // same span every painter writes, and `back` is a separate allocation.
    let surface = unsafe { core::slice::from_raw_parts_mut(target as *mut u32, words) };
    let mut wrote = false;
    for (dst, src) in surface.chunks_mut(row_words).zip(back.chunks(row_words)) {
        if dst != src {
            dst.copy_from_slice(src);
            wrote = true;
        }
    }
    // Drawn on another backing than the shown one, it differs from it; drawn
    // on the shown one, it is new where a row was written.
    wrote || apart.is_some() || shown.is_none()
}
