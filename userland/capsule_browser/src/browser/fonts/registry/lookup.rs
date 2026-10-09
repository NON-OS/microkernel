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

use nonos_toolkit::font::ttf::FontRef;

use super::super::key::{weight_of, without_weight};
use super::face::Face;
use super::FONTS;

/* Weights kept per variable face besides the one it loaded as, and the
 * bytes all faces may hold with those copies; past either a further weight
 * draws with the face as loaded. */
const MAX_WEIGHTS: usize = 4;
const MAX_ALL_BYTES: usize = 6 * 1024 * 1024;

/* Run `f` with the parsed face for `key`. The face borrows the registry entry
 * for the duration of the call. */
pub fn with_face<R>(key: u32, f: impl FnOnce(&FontRef) -> R) -> Option<R> {
    with_weighted(key, f).map(|(r, _)| r)
}

/// with_face, and whether the face drew the key's CSS weight itself: a
/// variable face set to that weight, made the first time it is asked for.
pub fn with_weighted<R>(key: u32, f: impl FnOnce(&FontRef) -> R) -> Option<(R, bool)> {
    let (base, w) = (without_weight(key), weight_of(key));
    let mut fonts = FONTS.lock();
    let at = |fonts: &[Face], w: u16| fonts.iter().position(|x| x.key == base && x.weight == w);
    let first = at(&fonts, 0)?;
    if fonts[first].wght.is_none_or(|d| d == w as f32) {
        return Some((f(fonts[first].font()), fonts[first].wght.is_some()));
    }
    let i = match at(&fonts, w) {
        Some(i) => Some(i),
        None if room(&fonts, base, fonts[first].data.len()) => Face::weighted(&fonts[first], w)
            .map(|v| {
                fonts.push(v);
                fonts.len() - 1
            }),
        None => None,
    };
    let i = i.unwrap_or(first);
    Some((f(fonts[i].font()), i != first))
}

/* Whether one more weight of `base`, `size` bytes, fits both limits. */
fn room(fonts: &[Face], base: u32, size: usize) -> bool {
    let bytes: usize = fonts.iter().map(|x| x.data.len()).sum();
    fonts.iter().filter(|x| x.key == base).count() <= MAX_WEIGHTS && bytes + size <= MAX_ALL_BYTES
}
