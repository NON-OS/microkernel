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

use super::hex::parse_hex;
use super::named::named;

pub(super) mod args;
mod by_name;
mod color_fn;
mod color_mix;
mod comp;
mod fmath;
mod hwb;
mod lab;
mod media;
mod mix_lerp;
mod mix_method;
mod mix_space;
mod oklab;
mod powers;
pub(super) mod rgbaf;
mod space_map;
pub(super) mod trig;

pub use media::media_query_matches;

/* Deepest parenthesis nesting a colour may carry: color-mix() and
 * light-dark() parse their arguments recursively. */
const MAX_DEPTH: i32 = 8;

/// A CSS colour as ARGB with its alpha kept: hex, rgb/hsl/hwb, lab/lch,
/// oklab/oklch, color(), color-mix(), light-dark(), named and system
/// colours. currentColor and anything unknown give None, which leaves the
/// property at the value it had.
pub fn parse_color(v: &str) -> Option<u32> {
    let s = v.trim();
    if let Some(hex) = s.strip_prefix('#') {
        return parse_hex(hex);
    }
    let Some(open) = s.find('(') else { return named(s) };
    let depth = s.bytes().try_fold(0i32, |d, b| {
        let d = d + (b == b'(') as i32 - (b == b')') as i32;
        (d <= MAX_DEPTH).then_some(d)
    });
    let inner = s[open + 1..].strip_suffix(')').filter(|_| depth == Some(0))?;
    by_name::by_name(&s[..open], inner)
}
