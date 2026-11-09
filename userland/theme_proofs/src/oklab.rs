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

//! Colour difference in Oklab, the space the categories are told apart in.

/*
 * Oklab (Ottosson, 2020) is built so that equal distances look equally different
 * across hue and lightness, which WCAG luminance is not: two colours of one
 * luminance are a contrast of 1:1 and can still be as far apart as blue and
 * orange. The matrices are the published ones, from linear sRGB.
 */
fn linear(channel: u8) -> f64 {
    let c = channel as f64 / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// The colour's Oklab coordinates, L, a and b.
pub fn oklab(colour: u32) -> [f64; 3] {
    let [r, g, b] = [16, 8, 0].map(|s| linear(((colour >> s) & 0xFF) as u8));
    let l = (0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b).cbrt();
    let m = (0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b).cbrt();
    let s = (0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b).cbrt();
    [
        0.210_454_255_3 * l + 0.793_617_785_0 * m - 0.004_072_046_8 * s,
        1.977_998_495_1 * l - 2.428_592_205_0 * m + 0.450_593_709_9 * s,
        0.025_904_037_1 * l + 0.782_771_766_2 * m - 0.808_675_766_0 * s,
    ]
}

/// The straight-line distance between two colours in Oklab.
pub fn distance(a: u32, b: u32) -> f64 {
    let (p, q) = (oklab(a), oklab(b));
    p.iter().zip(q).map(|(x, y)| (x - y) * (x - y)).sum::<f64>().sqrt()
}
