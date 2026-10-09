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

//! SGR: the pen's colours and attributes. Colours read the same whether a
//! program writes `38;2;r;g;b`, `38:2:r:g:b` or `38:2::r:g:b`.

use super::sgr_attr::plain;
use super::sgr_colour::{colour_from_group, colour_from_spread};
use super::state::Term;
use crate::cell::Pen;
use crate::params::Params;

impl Term {
    pub(super) fn sgr(&mut self, p: &Params) {
        let groups: alloc::vec::Vec<&[u16]> = p.groups().collect();
        let mut pen = self.scr_ref().cur.pen;
        if groups.is_empty() {
            pen = Pen { link: pen.link, ..Pen::default() };
        }
        let mut i = 0;
        while i < groups.len() {
            let g = groups[i];
            i += 1;
            if !matches!(g[0], 38 | 48 | 58) {
                plain(&mut pen, g);
                continue;
            }
            let colour = if g.len() > 1 {
                colour_from_group(g)
            } else {
                // The semicolon form spreads over the next groups.
                let (c, used) = colour_from_spread(&groups[i..]);
                i += used;
                c
            };
            /*
             * Underline colour (58) is read so its numbers are not taken for
             * other attributes, and not drawn.
             */
            match (g[0], colour) {
                (38, Some(c)) => pen.fg = c,
                (48, Some(c)) => pen.bg = c,
                _ => {}
            }
        }
        self.scr().cur.pen = pen;
    }
}
