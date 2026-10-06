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

use alloc::vec::Vec;

use super::parse::{parse_linear, Linear};
use super::radial::parse_radial;

/* A parsed gradient: a linear one with its angle and stops, or a radial one
 * drawn as a circle from the box centre out to its corners. */
pub(super) enum Shape {
    Linear(Linear),
    Radial(Vec<(u32, f32)>),
}

/* True when the background value is a gradient function this module owns. */
pub(crate) fn is_gradient(src: &str) -> bool {
    src.starts_with("linear-gradient(") || src.starts_with("radial-gradient(")
}

/* None when the value is not a gradient this module can draw. */
pub(super) fn parse(src: &str) -> Option<Shape> {
    if let Some(g) = parse_linear(src) {
        return Some(Shape::Linear(g));
    }
    parse_radial(src).map(Shape::Radial)
}
