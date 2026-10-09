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
            "mask" | "mask-image" | "-webkit-mask" | "-webkit-mask-image" => {
                /* A url mask takes the background image slot; a gradient
                 * mask is not drawn, and the box paints unmasked. */
                let url = bg_url("background-image", v).filter(|u| !u.contains("gradient("));
                if self.c.fx.mask || url.is_some() {
                    self.bg = url.clone();
                }
                self.c.fx.mask = url.is_some();
                self.c.fx.fade = crate::browser::layout::fade_table::fade_id(v);
            }
            "mask-composite" | "-webkit-mask-composite" => {
                let first = v.split(',').next().map(str::trim);
                self.c.fx.fade_isect = matches!(first, Some("intersect" | "source-in"));
            }
            "content" => self.content = Some(String::from(v)),
            "counter-reset" => self.counters[0] = Some(String::from(v)),
            "counter-increment" => self.counters[1] = Some(String::from(v)),
            "counter-set" => self.counters[2] = Some(String::from(v)),
            /* bolder and lighter step from the parent's weight. */
            "font-weight" => {
                let w = crate::browser::fonts::weight_of(self.parent_key);
                self.c.font_key = crate::browser::fonts::weighted(self.c.font_key, w);
            }
            _ => {}
        }
        apply_decl(&mut self.c, name, v, self.parent_fs);
        grid_decl(&mut self.grid, name, v, self.c.font_size_px);
    }
}
