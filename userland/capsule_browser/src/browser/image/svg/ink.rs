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

use super::color::parse_paint;
use super::defs::{ref_id, Defs};
use super::state::Paint;

/// A fill or stroke paint: a colour, or the gradient at an index of Defs.
#[derive(Clone, Copy)]
pub(super) enum Ink {
    Solid(u32),
    Grad(usize),
}

/// A paint value: a colour, a gradient reference (its fallback colour, or
/// none, when the id is not a gradient), or none. None when unreadable,
/// which keeps the inherited paint.
pub(super) fn ink(v: &str, defs: &Defs, current: u32) -> Option<Option<Ink>> {
    if let Some(id) = v.trim().starts_with("url(").then(|| ref_id(v)).flatten() {
        if let Some(i) = defs.gradient(id) {
            return Some(Some(Ink::Grad(i)));
        }
        let fallback = v.split_once(')').map_or("", |(_, rest)| rest);
        return Some(parse_paint(fallback, current).flatten().map(Ink::Solid));
    }
    parse_paint(v, current).map(|c| c.map(Ink::Solid))
}

/// Layer an element's color, fill and stroke, read through `prop`, onto
/// `p`: currentColor in fill and stroke means the color just set.
pub(super) fn inks<'a>(p: &mut Paint, prop: impl Fn(&str) -> Option<&'a str>, defs: &Defs) {
    if let Some(c) = prop("color").and_then(|v| parse_paint(v, p.color)) {
        p.color = c.unwrap_or(0);
    }
    if let Some(paint) = prop("fill").and_then(|v| ink(v, defs, p.color)) {
        p.fill = paint;
    }
    if let Some(paint) = prop("stroke").and_then(|v| ink(v, defs, p.color)) {
        p.stroke = paint;
    }
}
