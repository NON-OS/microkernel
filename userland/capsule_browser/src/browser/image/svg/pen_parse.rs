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

use super::num::{num_list, parse_len};
use super::pen::Pen;

impl Pen {
    /// Layer this element's stroke properties, read through `prop`, on.
    pub fn derive<'a>(&mut self, prop: impl Fn(&str) -> Option<&'a str>) {
        let word = |n: &str| prop(n).map(|v| v.trim());
        if let Some(w) = prop("stroke-width").and_then(parse_len).filter(|w| *w >= 0.0) {
            self.width = w;
        }
        match word("stroke-linecap") {
            Some("butt") => self.cap = 0,
            Some("round") => self.cap = 1,
            Some("square") => self.cap = 2,
            _ => {}
        }
        match word("stroke-linejoin") {
            Some("miter") | Some("miter-clip") | Some("arcs") => self.join = 0,
            Some("round") => self.join = 1,
            Some("bevel") => self.join = 2,
            _ => {}
        }
        if let Some(m) = prop("stroke-miterlimit").and_then(parse_len).filter(|m| *m >= 1.0) {
            self.miter = m;
        }
        if let Some(o) = prop("stroke-dashoffset").and_then(parse_len) {
            self.offset = o;
        }
        if let Some(v) = word("stroke-dasharray") {
            let list = num_list(v);
            /* An odd list repeats to make it even; none, a negative length
             * or an all-zero pattern draws solid. */
            let n = if list.len() % 2 == 1 { 2 * list.len() } else { list.len() };
            let usable =
                n <= 16 && list.iter().all(|d| *d >= 0.0) && list.iter().sum::<f32>() > 0.0;
            self.dashes = if usable { n } else { 0 };
            (0..self.dashes).for_each(|i| self.dash[i] = list[i % list.len()]);
        }
    }
}
