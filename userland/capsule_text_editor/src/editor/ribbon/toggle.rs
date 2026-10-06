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

//! What a ribbon control does to the document. The target value is read from
//! the caret first and then written across the whole selection, so a mixed
//! selection resolves to one state instead of flipping each run apart.

use super::items::{HEADINGS, SIZES};
use super::snapshot::ribbon_state;
use crate::doc::style::Family;
use crate::editor::state::State;
use crate::editor::style_marks::Mark;

const NEED_SEL: &[u8] = b"select text to format";

impl State {
    pub(in crate::editor) fn apply_toggle(&mut self, t: usize) {
        let on = !ribbon_state(self).flags[t];
        let st = self;
        let applied = match t {
            0 => st.restyle_sel(&move |m: &mut Mark| m.bold = Some(on)),
            1 => st.restyle_sel(&move |m: &mut Mark| m.italic = Some(on)),
            2 => st.restyle_sel(&move |m: &mut Mark| m.underline = Some(on)),
            3 => st.restyle_sel(&move |m: &mut Mark| m.strike = Some(on)),
            _ => st.restyle_sel(&move |m: &mut Mark| m.accent = Some(on)),
        };
        if !applied {
            st.status = NEED_SEL;
        }
    }

    pub(in crate::editor) fn apply_pill_row(&mut self, pill: usize, row: usize) {
        let st = self;
        let applied = match pill {
            0 => match HEADINGS.get(row) {
                Some(&(_, level)) => {
                    st.set_heading(level);
                    true
                }
                None => true,
            },
            1 => {
                let family = if row == 1 { Family::Mono } else { Family::Sans };
                st.restyle_sel(&move |m: &mut Mark| m.family = Some(family))
            }
            _ => match SIZES.get(row) {
                Some(&px) => st.restyle_sel(&move |m: &mut Mark| m.size = Some(px as u8)),
                None => true,
            },
        };
        if !applied {
            st.status = NEED_SEL;
        }
    }
}
