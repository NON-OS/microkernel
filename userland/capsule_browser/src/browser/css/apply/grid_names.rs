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

use crate::browser::css::grid_lines::{area_rows, col_line_names};
use crate::browser::css::grid_spec::GridSpec;

use super::grid_halves::halves;

/* The names a grid container's template carries into its side spec: the
 * [named] column lines and the template areas, from the longhands or from
 * the grid-template and grid shorthands (areas as the quoted rows, lines
 * from the columns half). A value naming nothing clears names an earlier
 * declaration gave, as the cascade's later-wins order requires, but makes
 * no spec where there was none. Returns false for other properties. */
pub(in crate::browser::css) fn template_names(
    spec: &mut Option<GridSpec>,
    name: &str,
    value: &str,
    em: u32,
) -> bool {
    let (lines, areas) = match name {
        "grid-template-columns" => (Some(col_line_names(value, em)), None),
        "grid-template-areas" => (None, Some(area_rows(value))),
        "grid-template" | "grid" => {
            let cols = halves(value.trim()).1.unwrap_or("");
            (Some(col_line_names(cols, em)), Some(area_rows(value)))
        }
        _ => return false,
    };
    let named = lines.as_ref().is_some_and(|l| !l.is_empty())
        || areas.as_ref().is_some_and(|a| !a.is_empty());
    if named || spec.is_some() {
        let s = GridSpec::ensure(spec);
        if let Some(l) = lines {
            s.col_lines = l;
        }
        if let Some(a) = areas {
            s.areas = a;
        }
    }
    true
}
