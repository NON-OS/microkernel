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

use crate::browser::css::hsl_fn::parse_hsl;
use crate::browser::css::rgb_fn::parse_rgb;

/// A colour function by its name, given the text inside its parentheses.
pub(super) fn by_name(name: &str, inner: &str) -> Option<u32> {
    let is = |n: &str| name.eq_ignore_ascii_case(n);
    if is("rgb") || is("rgba") {
        parse_rgb(inner)
    } else if is("hsl") || is("hsla") {
        parse_hsl(inner)
    } else if is("hwb") {
        super::hwb::parse_hwb(inner)
    } else if is("lab") || is("lch") {
        super::lab::parse_lab(inner, is("lch"))
    } else if is("oklab") || is("oklch") {
        super::oklab::parse_oklab(inner, is("oklch"))
    } else if is("color") {
        super::color_fn::parse_color_fn(inner)
    } else if is("color-mix") {
        super::color_mix::parse_color_mix(inner)
    } else if is("light-dark") {
        /* Pages here render in the light scheme. */
        super::parse_color(crate::browser::css::calc::split_top::items(inner).next()?)
    } else {
        None
    }
}
