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

use super::super::ops::state::Builder;

impl Builder {
    /// The adoption agency algorithm (13.2.6.4.7) for an end tag named
    /// `subject`. Returns true when the tag is to be handled as "any other
    /// end tag" instead.
    ///
    /// This is what turns `<b>1<p>2</b>3</p>` into a bold "1", a paragraph
    /// holding a bold "2" and a plain "3", the tree every browser builds.
    pub(in super::super) fn adoption(&mut self, subject: &str) -> bool {
        let cur = self.cur();
        if self.is(cur, subject) && !self.in_fmt(cur) {
            self.pop();
            return false;
        }
        for _ in 0..8 {
            let Some(fe) = self.fmt_last_named(subject) else {
                return true;
            };
            if !self.on_stack(fe) {
                self.remove_fmt(fe);
                return false;
            }
            if !self.node_in_scope(fe) {
                return false;
            }
            let Some(fe_pos) = self.open.iter().rposition(|&x| x == fe) else {
                return false;
            };
            let furthest =
                (fe_pos + 1..self.open.len()).find(|&i| self.special_at(i)).map(|i| self.open[i]);
            let Some(fb) = furthest else {
                while let Some(id) = self.pop() {
                    if id == fe {
                        break;
                    }
                }
                self.remove_fmt(fe);
                return false;
            };
            if !self.adopt(fe, fe_pos, fb) {
                return false;
            }
        }
        false
    }
}
