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

use crate::browser::css::decl::Decl;

use super::super::order::Order;
use super::plan::{plan, Step};
use super::Styling;

impl Styling<'_> {
    /* Apply `order` in two passes: font-size first, since em, ex and
     * percentage lengths and line heights of the same box resolve
     * against it, then every other property. Custom properties were
     * cascaded into the scope already. */
    pub fn run(&mut self, order: &Order) {
        let mut all: Vec<&Decl> = Vec::new();
        order.each(0, &mut |d| all.push(d));
        let steps = plan(&all, order);
        for font in [true, false] {
            for (i, d) in all.iter().enumerate() {
                if (d.name == "font-size") != font || d.flags & Decl::CUSTOM != 0 {
                    continue;
                }
                match steps.get(i).copied().unwrap_or(Step::Apply) {
                    Step::Apply => self.apply(d),
                    Step::Put(v) => self.apply_value(&d.name, v),
                    Step::Skip => {}
                }
            }
        }
    }
}
