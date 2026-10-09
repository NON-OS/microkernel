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
//! A slash inside a colour function is part of the colour; only a
//! top-level slash joins a position to a size.

use super::bg_shadow_tests::page;

#[test]
fn a_slash_alpha_colour_survives_the_background_shorthand() {
    let bg = |v: &str| page(&format!("#a{{background:{v}}}")).frag("a").bg;
    assert_eq!(bg("rgb(0 0 0 / 50%)"), 0x8000_0000);
    assert_eq!(bg("hsl(0 100% 50% / .5) url(x.png) no-repeat"), 0x80FF_0000);
    assert_eq!(bg("#fff url(a.png) center/cover"), 0xFFFF_FFFF);
    assert_eq!(bg("url(a.png) 0 0/10px rgb(1 2 3/1)"), 0xFF01_0203);
}
