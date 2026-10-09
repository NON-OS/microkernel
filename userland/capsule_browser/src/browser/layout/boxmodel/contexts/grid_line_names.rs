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

use crate::browser::css::GridSpec;

use super::super::tree::GridPlace;

/* A container's line names on one axis: the [named] column lines, and the
 * lines each template area implies (name-start, name-end), on either axis. */
#[derive(Clone, Copy)]
pub(in super::super) struct Names<'a> {
    pub cont: &'a GridSpec,
    pub cols: bool,
}

impl Names<'_> {
    /* The 1-based line an explicit [name] opens (columns only). */
    pub(in super::super) fn find(&self, name: &str) -> Option<i16> {
        let hit = self.cont.col_lines.iter().find(|(n, _)| self.cols && n == name);
        hit.map(|(_, i)| *i as i16 + 1)
    }

    /* The line a named start or end refers to: name-start (name-end), from
     * the lines or an area, else a line called just name. */
    pub(in super::super) fn line(&self, name: &str, end: bool) -> Option<i16> {
        let suffixed = [name, if end { "-end" } else { "-start" }].concat();
        let cols = self.cols;
        let area = self.cont.areas.iter().enumerate().flat_map(|(r, row)| {
            let hits = row.iter().enumerate().filter(|(_, c)| c.as_str() == name);
            hits.map(move |(c, _)| if cols { c } else { r })
        });
        let edge = if end { area.max().map(|v| v + 1) } else { area.min() };
        self.find(&suffixed)
            .or(edge.map(|v| v.min(i16::MAX as usize - 1) as i16 + 1))
            .or_else(|| self.find(name))
    }
}

/* grid-area: <name> spans the cells the template areas give the name, or
 * failing that, the columns between its name-start and name-end lines. */
pub(in super::super) fn area_place(cont: &GridSpec, name: &str) -> Option<GridPlace> {
    let mut hit: Option<[usize; 4]> = None;
    for (r, row) in cont.areas.iter().enumerate() {
        for (c, _) in row.iter().enumerate().filter(|(_, cell)| cell.as_str() == name) {
            let h = hit.get_or_insert([r, c, r, c]);
            *h = [h[0].min(r), h[1].min(c), h[2].max(r), h[3].max(c)];
        }
    }
    let line = |v: usize| v.min(i16::MAX as usize - 2) as i16 + 1;
    if let Some([r0, c0, r1, c1]) = hit {
        return Some(GridPlace {
            col: [line(c0), line(c1 + 1)],
            row: [line(r0), line(r1 + 1)],
            col_span: 1,
            row_span: 1,
        });
    }
    let names = Names { cont, cols: true };
    let start = names.find(&[name, "-start"].concat())?;
    let end = names.find(&[name, "-end"].concat()).unwrap_or(0);
    Some(GridPlace { col: [start, end], row: [0, 0], col_span: 1, row_span: 1 })
}
