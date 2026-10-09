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

use alloc::borrow::Cow;
use alloc::string::String;

use crate::browser::html::tokenizer::Tag;

use super::svg_attrs::svg_attr;
use super::svg_tags::svg_tag;

/// Adjust MathML attributes: the one camel-case name.
pub fn adjust_mathml(t: &mut Tag) {
    for (name, _) in t.attrs.iter_mut() {
        if name == "definitionurl" {
            *name = String::from("definitionURL");
        }
    }
}

/// Adjust SVG attributes: the tokenizer lowercased every name, and SVG's
/// own camel-case ones (`viewBox`, `preserveAspectRatio`) get their case
/// back. Namespaced ones such as `xlink:href` keep their written name.
pub fn adjust_svg(t: &mut Tag) {
    for (name, _) in t.attrs.iter_mut() {
        if let Some(fixed) = svg_attr(name) {
            *name = String::from(fixed);
        }
    }
}

/// The case an SVG element's name is written in (`clipPath`,
/// `linearGradient`, `foreignObject`).
pub fn svg_tag_case(name: &mut Cow<'_, str>) {
    if let Some(fixed) = svg_tag(name) {
        *name = Cow::Borrowed(fixed);
    }
}
