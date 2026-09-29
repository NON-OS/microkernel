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

use crate::browser::css::apply::apply_decl;
use crate::browser::css::bg_url::bg_url;
use crate::browser::css::decl::Decl;
use crate::browser::css::grid_area_decl::grid_decl;
use crate::browser::css::vars::substitute;

use super::Styling;

impl Styling<'_> {
    /* Apply one declaration: var() resolved first (a value that fails to
     * resolve is invalid and changes nothing), then the values kept
     * beside the style, then the style and the grid data. */
    pub(super) fn apply(&mut self, d: &Decl) {
        let v = match d.needs_resolve() {
            true => match substitute(&d.value, &mut self.vars) {
                Some(v) => v,
                None => return,
            },
            false => Cow::Borrowed(d.value.as_str()),
        };
        self.apply_value(d.name.as_str(), &v);
    }

    /* Apply `v`, free of var(), as the value of property `name`. */
    pub(super) fn apply_value(&mut self, name: &str, v: &str) {
        match name {
            "background" | "background-image" => self.bg = bg_url(name, v),
            "content" => self.content = Some(String::from(v)),
            "counter-reset" => self.counters[0] = Some(String::from(v)),
            "counter-increment" => self.counters[1] = Some(String::from(v)),
            "counter-set" => self.counters[2] = Some(String::from(v)),
            _ => {}
        }
        apply_decl(&mut self.c, name, v, self.parent_fs);
        grid_decl(&mut self.grid, name, v, self.c.font_size_px);
    }
}
