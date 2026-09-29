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

//! Known answers for the CSS Color 4/5 syntaxes, each ARGB value read back
//! from Chromium (canvas pixels for opaque colours, computed style for the
//! translucent ones). Every channel must land within one step of it.

use crate::browser::css::color::parse_color;

const CHROMIUM: &[(&str, u32)] = &[
    ("rgba(242,243,247,.74)", 0xBDF2_F3F7),
    ("rgb(242 243 247 / 74%)", 0xBDF2_F3F7),
    ("rgba(10,11,15,.34)", 0x570A_0B0F),
    ("rgb(0 0 0 / 50%)", 0x8000_0000),
    ("#0a0b0d80", 0x800A_0B0D),
    ("#f00c", 0xCCFF_0000),
    ("hsl(210 40% 50%)", 0xFF4D_80B3),
    ("hsla(120, 100%, 25%, 0.3)", 0x4D00_8000),
    ("hsl(330deg 60% 70%)", 0xFFE0_85B3),
    ("hsl(-120 100% 50%)", 0xFF00_00FF),
    ("hwb(200 20% 30%)", 0xFF33_88B3),
    ("hwb(0 60% 60%)", 0xFF80_8080),
    ("oklch(0.7 0.15 250)", 0xFF4B_A3F7),
    ("oklch(62.8% 0.2577 29.23)", 0xFFFF_0000),
    ("oklab(0.5 -0.1 0.1)", 0xFF3C_740A),
    ("oklch(0.985 0.002 247.839)", 0xFFF9_FAFB),
    ("oklch(0.21 0.034 264.665)", 0xFF10_1828),
    ("lab(50 40 -20)", 0xFFAB_5A9A),
    ("lch(60 50 120)", 0xFF76_9C3E),
    ("lab(90% 10% -30%)", 0xFFDD_DCFF),
    ("color(display-p3 1 0.5 0)", 0xFFFF_7600),
    ("color(srgb 0.2 0.4 0.6)", 0xFF33_6699),
    ("color(srgb-linear 0.2 0.4 0.6)", 0xFF7C_AACB),
    ("color(xyz 0.3 0.4 0.5)", 0xFF5C_B8B5),
    ("color-mix(in srgb, #66ffff 22%, transparent)", 0x3866_FFFF),
    ("color-mix(in srgb, red, blue)", 0xFF80_0080),
    ("color-mix(in oklab, red 30%, blue)", 0xFF5D_4BC8),
    ("color-mix(in oklch, red, blue)", 0xFFBA_00C2),
    ("color-mix(in oklch longer hue, red, blue)", 0xFF00_9300),
    ("color-mix(in lch, white, blue)", 0xFFAF_89FF),
    ("color-mix(in hsl, red 40%, lime)", 0xFFCC_FF00),
    ("color-mix(in srgb-linear, #000, #fff)", 0xFFBC_BCBC),
    ("color-mix(in oklab, oklch(0.7 0.15 250) 50%, transparent)", 0x804B_A3F7),
    ("color-mix(in srgb, red 20%, blue 20%)", 0x6680_0080),
    ("color-mix(in lab, #123456, #abcdef 70%)", 0xFF7C_9CBE),
    ("rebeccapurple", 0xFF66_3399),
    ("LightGoldenrodYellow", 0xFFFA_FAD2),
    ("CanvasText", 0xFF00_0000),
    ("light-dark(#111, #eee)", 0xFF11_1111),
    ("rgb(300 -5 127.5)", 0xFFFF_0080),
];

#[test]
fn every_syntax_matches_chromium_within_one_step() {
    for &(css, want) in CHROMIUM {
        let got = parse_color(css).unwrap_or_else(|| panic!("{css} did not parse"));
        let off = (0..4).map(|i| ((got >> (8 * i)) & 0xFF).abs_diff((want >> (8 * i)) & 0xFF));
        assert!(off.max().unwrap_or(0) <= 1, "{css}: got {got:08x}, Chromium {want:08x}");
    }
}
