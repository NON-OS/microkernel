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

//! Malformed colours are rejected, leaving the property as it was.

use crate::browser::css::color::parse_color;

#[test]
fn malformed_colours_are_rejected() {
    for bad in [
        "rgb(1,2)",
        "rgb(1,2,3,4,5)",
        "rgb(1 2 3) x",
        "oklch(0.5 0.1)",
        "color(foo 1 2 3)",
        "color-mix(in srgb, red)",
        "color-mix(in nowhere, red, blue)",
        "color-mix(in srgb, red -5%, blue)",
        "color-mix(in srgb, red 0%, blue 0%)",
        "rgb(nan, 0, 0)",
        "hsl(inf 1% 1%)",
        "#12",
        "notacolour",
        "rgb(((((((((((1,2,3)))))))))))",
    ] {
        assert!(parse_color(bad).is_none(), "{bad} should not parse");
    }
}
