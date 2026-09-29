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

use crate::browser::css::calc::split_top::words;
use crate::browser::css::computed::{Align, Computed};

use super::align_kw::{justify_kw, self_kw};

/* Box alignment: justify-content and align-items on the container, the
 * justify-items default for its grid items, and each item's own
 * justify-self and align-self, with the place-* shorthands that set an
 * align and a justify value at once. Anything else goes on down the chain. */
pub(super) fn apply_align(c: &mut Computed, name: &str, value: &str) -> bool {
    let v = value.trim();
    match name {
        "justify-content" => {
            if let Some(j) = justify_kw(v) {
                c.justify = j;
            }
        }
        "align-items" => set(&mut c.align, v),
        "justify-items" => set_opt(&mut c.justify_items, v),
        "justify-self" => set_opt(&mut c.justify_self, v),
        "align-self" => set_opt(&mut c.align_self, v),
        "place-items" | "place-self" | "place-content" => {
            let mut it = words(v);
            let (a, j) = match (it.next(), it.next()) {
                (Some(a), Some(j)) => (a, j),
                (Some(a), None) => (a, a),
                _ => return true,
            };
            if name == "place-items" {
                set(&mut c.align, a);
                set_opt(&mut c.justify_items, j);
            } else if name == "place-self" {
                set_opt(&mut c.align_self, a);
                set_opt(&mut c.justify_self, j);
            } else if let Some(k) = justify_kw(j) {
                /* align-content is not laid out; only the justify half. */
                c.justify = k;
            }
        }
        _ => return super::direction::apply_direction(c, name, value),
    }
    true
}

/* align-items: normal takes the stretch it computes to for flex and grid. */
fn set(slot: &mut Align, v: &str) {
    if let Some(a) = self_kw(v) {
        *slot = a.unwrap_or(Align::Stretch);
    }
}

fn set_opt(slot: &mut Option<Align>, v: &str) {
    if let Some(a) = self_kw(v) {
        *slot = a;
    }
}
