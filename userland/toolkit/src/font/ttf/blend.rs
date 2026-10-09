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

/* Full weight: coverage 255 times colour alpha 255. */
const FULL: u32 = 255 * 255;

/*
 * Composite `argb` over a pixel at glyph coverage `cov` (0..=255). The
 * colour's own alpha scales the coverage, so translucent text fades instead
 * of ignoring its opacity, and an opaque surface stays opaque. A pixel that
 * is not opaque, such as the clear overlay a shell draws its labels on, takes
 * the straight alpha path in `over_clear` instead.
 *
 * In integers: the weight is cov x alpha out of 255 x 255 and each channel
 * rounds to nearest, which is what the per-pixel float mix this replaces
 * computed, without a float per pixel (a hundred times slower than this
 * under emulation). For an opaque colour the two agree on every input.
 */
#[inline]
pub(super) fn mix(dst: u32, argb: u32, cov: u8) -> u32 {
    if dst >> 24 != 0xff {
        return over_clear(dst, argb, cov);
    }
    let alpha = argb >> 24;
    let a = cov as u32 * alpha;
    if a == FULL {
        return argb | 0xff00_0000;
    }
    let ch = |shift: u32| {
        let (s, d) = ((argb >> shift) & 0xff, (dst >> shift) & 0xff);
        let t = s * a + d * (FULL - a) + FULL / 2;
        if alpha != 255 && matches!(t % FULL, 0 | NEAR_HALF..) {
            return near_half(s, d, cov, alpha);
        }
        t / FULL
    };
    0xff00_0000 | (ch(16) << 16) | (ch(8) << 8) | ch(0)
}

/* Where the rounded sum's remainder lands when the exact value is one of the
three fractions nearest a half (32511 to 32513 over 65025): here and above,
or wrapped to 0. */
const NEAR_HALF: u32 = FULL - 2;

/*
 * A translucent channel whose exact value lies within 2/65025 of a half. The
 * float mix rounded these by its own error, not by the exact value, so the
 * same float expression settles them, and text drawn before and after the
 * move to integers is identical to the bit. About 3 in 100000 translucent
 * samples take this path; opaque colours never do.
 */
fn near_half(s: u32, d: u32, cov: u8, alpha: u32) -> u32 {
    let a = cov as f32 / 255.0 * alpha as f32 / 255.0;
    (s as f32 * a + d as f32 * (1.0 - a) + 0.5) as u32
}

/*
 * Coverage over a pixel that is not opaque, in straight alpha. Treating such
 * a pixel as opaque blended the text colour toward whatever colour the clear
 * pixel held, black on a cleared overlay, and then marked the result opaque.
 * Every edge of every glyph became a dark opaque ring that the compositor
 * laid over the wallpaper, which is what made desktop labels unreadable on
 * light and mid tone wallpapers. Here the edge keeps the text colour and
 * carries its coverage as alpha, so the compositor blends it with whatever
 * lies underneath.
 */
fn over_clear(dst: u32, argb: u32, cov: u8) -> u32 {
    let sa = (cov as u32 * (argb >> 24) + 127) / 255;
    if sa == 0 {
        return dst;
    }
    let da = dst >> 24;
    let ia = 255 - sa;
    let oa = sa + (da * ia + 127) / 255;
    let den = oa * 255;
    let ch = |shift: u32| {
        let (s, d) = ((argb >> shift) & 0xff, (dst >> shift) & 0xff);
        (s * sa * 255 + d * da * ia + den / 2) / den
    };
    oa << 24 | ch(16) << 16 | ch(8) << 8 | ch(0)
}
