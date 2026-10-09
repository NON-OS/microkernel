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

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::apply::grid_names::template_names;
use super::grid_spec::GridSpec;

/* Capture the named-grid and placement declarations into the per-node
 * side spec. Runs beside the normal property apply for every declaration,
 * so cascade order (later wins) holds; anything not grid- leaves at once. */
pub(super) fn grid_decl(spec: &mut Option<GridSpec>, name: &str, value: &str, em: u32) {
    if !name.starts_with("grid") || template_names(spec, name, value, em) {
        return;
    }
    let parts: Vec<&str> = value.split('/').map(str::trim).take(5).collect();
    let one = |i: usize| parts.get(i).filter(|p| !p.is_empty()).map(|p| p.to_string());
    match name {
        "grid-area" if parts.len() > 4 => {}
        "grid-area" => {
            /* A lone name is the area; with slashes, each part is a line and
             * an omitted end repeats a named start, as CSS defines. */
            let s = GridSpec::ensure(spec);
            let named = |p: &Option<String>| p.clone().filter(|v| GridSpec::is_ident(v));
            (s.area, s.row_start, s.col_start) = (None, one(0), one(1));
            if parts.len() == 1 && s.row_start.as_deref().is_some_and(GridSpec::is_ident) {
                (s.area, s.row_start) = (s.row_start.take(), None);
            }
            s.row_end = one(2).or_else(|| named(&s.row_start));
            s.col_end = one(3).or_else(|| named(&s.col_start));
        }
        "grid-column" | "grid-row" if parts.len() > 2 => {}
        "grid-column" | "grid-row" => {
            let (s, end) = (
                GridSpec::ensure(spec),
                one(1).or_else(|| one(0).filter(|v| GridSpec::is_ident(v))),
            );
            match name {
                "grid-column" => (s.col_start, s.col_end) = (one(0), end),
                _ => (s.row_start, s.row_end) = (one(0), end),
            }
        }
        "grid-column-start" => GridSpec::ensure(spec).col_start = one(0),
        "grid-column-end" => GridSpec::ensure(spec).col_end = one(0),
        "grid-row-start" => GridSpec::ensure(spec).row_start = one(0),
        "grid-row-end" => GridSpec::ensure(spec).row_end = one(0),
        _ => {}
    }
}
