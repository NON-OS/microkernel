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

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_toolkit::font::ttf::{self, FontRef};
use spin::Mutex;

mod face;

use face::Face;

/* Loaded page fonts keyed by family hash. A global rather than a threaded
 * parameter because glyph measurement happens deep inside layout, which never
 * sees the page state; the capsule is single threaded so the lock is only a
 * formality. Cleared on navigation with the rest of the page. */
static FONTS: Mutex<Vec<Face>> = Mutex::new(Vec::new());

const MAX_FONTS: usize = 12;
const MAX_FONT_BYTES: usize = 2 * 1024 * 1024;

/* Validate and install a fetched face under its family key, parsing it once
here: every measure and draw after this reuses the parsed face. Returns true
when the face parsed and text using it should relayout. */
pub fn install(key: u32, bytes: Vec<u8>) -> bool {
    if key == 0 || bytes.len() > MAX_FONT_BYTES {
        return false;
    }
    let mut fonts = FONTS.lock();
    if fonts.len() >= MAX_FONTS || fonts.iter().any(|f| f.key == key) {
        return false;
    }
    match Face::parse(key, Box::from(bytes)) {
        Some(face) => {
            fonts.push(face);
            true
        }
        None => false,
    }
}

/* Forget every page face. The glyph cache knows faces by the address of
their data, which is freed here and may hold the next page's face, so the
cache is emptied with them. */
pub fn clear() {
    let faces = core::mem::take(&mut *FONTS.lock());
    drop(faces);
    ttf::clear_glyph_cache();
}

/* Run `f` with the parsed face for `key`. The face borrows the registry entry
 * for the duration of the call. */
pub fn with_face<R>(key: u32, f: impl FnOnce(&FontRef) -> R) -> Option<R> {
    if key == 0 {
        return None;
    }
    let fonts = FONTS.lock();
    let face = fonts.iter().find(|x| x.key == key)?;
    Some(f(face.font()))
}
