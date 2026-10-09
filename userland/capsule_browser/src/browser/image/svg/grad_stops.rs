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

use super::attr::attr;
use super::defs::{ref_id, Defs};

/// How far an href chain of gradients is followed.
const MAX_HOPS: usize = 8;

/// Attribute `name` of gradient `i`, or of the gradients its href chain
/// reaches when it does not set it.
pub(super) fn get<'a>(defs: &Defs<'a>, mut i: usize, name: &str) -> Option<&'a str> {
    for _ in 0..MAX_HOPS {
        let g = &defs.grads[i].1;
        if let Some(v) = attr(g.attrs, name) {
            return Some(v);
        }
        i = defs.gradient(ref_id(attr(g.attrs, "href")?)?)?;
    }
    None
}

/// The stops gradient `i` paints with: its own, or those of the first
/// gradient along its href chain that has any.
pub(super) fn stops_of<'d>(defs: &'d Defs, mut i: usize) -> &'d [(f32, u32)] {
    for _ in 0..MAX_HOPS {
        let g = &defs.grads[i].1;
        if !g.stops.is_empty() {
            return &g.stops;
        }
        let Some(next) = attr(g.attrs, "href").and_then(ref_id).and_then(|id| defs.gradient(id))
        else {
            break;
        };
        i = next;
    }
    &[]
}
