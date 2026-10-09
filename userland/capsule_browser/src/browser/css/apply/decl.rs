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

mod logical;

use crate::browser::css::computed::Computed;

use super::align::apply_align;
use super::border::apply_border;
use super::display::apply_display;
use super::flex::apply_flex;
use super::grid::apply_grid;
use super::list::apply_list;
use super::margin::apply_margin;
use super::padding::apply_padding;
use super::paint::apply_paint;
use super::position::apply_position;
use super::sizing::apply_sizing;
use super::text::apply_text;

/* Apply one declaration, its value free of var() and light-dark(), to a
 * computed style. Each domain applier claims the properties it owns and
 * returns true; an unknown property falls through to false, which is
 * also how @supports asks whether this engine styles a property. */
pub fn apply_decl(c: &mut Computed, name: &str, value: &str, parent_fs: u32) -> bool {
    let fs = c.font_size_px;
    let value = match name {
        "text-align" => text_align(c, value),
        _ => value,
    };
    if let Some(done) = logical::apply_logical(c, name, value, fs) {
        return done;
    }
    apply_text(c, name, value, fs, parent_fs)
        || apply_margin(c, name, value, fs)
        || apply_padding(c, name, value, fs)
        || apply_border(c, name, value, fs)
        || apply_sizing(c, name, value, fs)
        || apply_display(c, name, value)
        || apply_flex(c, name, value, fs)
        || apply_align(c, name, value)
        || apply_grid(c, name, value, fs)
        || apply_list(c, name, value)
        || apply_position(c, name, value, fs)
        || super::float::apply_float(c, name, value)
        || apply_paint(c, name, value, fs)
        || super::shadow::apply_shadow(c, name, value, fs)
}

/* text-align: -webkit-center aligns text as center does and also centres
 * the tables inside; any other keyword ends that. */
fn text_align<'v>(c: &mut Computed, value: &'v str) -> &'v str {
    let v = value.trim();
    if v.eq_ignore_ascii_case("-webkit-center") {
        c.table.center_blocks = true;
        return "center";
    }
    if matches!(v, "left" | "start" | "center" | "right" | "end") {
        c.table.center_blocks = false;
    }
    value
}
