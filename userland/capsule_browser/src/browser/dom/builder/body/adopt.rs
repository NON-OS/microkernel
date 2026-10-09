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

use super::super::super::node::Ns;
use super::super::ops::link::Loc;
use super::super::ops::state::Builder;

impl Builder {
    /// The adoption agency's steps once the furthest block is known: the
    /// inner loop that clones the formatting elements between the two.
    /// False when the node cap stopped it.
    pub(in super::super) fn adopt(&mut self, fe: usize, fe_pos: usize, fb: usize) -> bool {
        let common = self.open[fe_pos - 1];
        let mut bookmark = self.fmt_index(fe).unwrap_or(self.fmt.len());
        let mut pos = self.open.iter().rposition(|&x| x == fb).unwrap_or(fe_pos + 1);
        let mut last = fb;
        for inner in 1.. {
            pos -= 1;
            let node = self.open[pos];
            if node == fe {
                break;
            }
            if inner > 3 {
                if let Some(j) = self.fmt_index(node) {
                    bookmark -= usize::from(j < bookmark);
                    self.remove_fmt(node);
                }
            }
            let Some(j) = self.fmt_index(node) else {
                self.remove_open(node);
                continue;
            };
            let Some(clone) = self.create(self.token_of(node), Ns::Html) else {
                return false;
            };
            self.replace_fmt(j, node, clone);
            self.replace_open(node, clone);
            if last == fb {
                bookmark = j + 1;
            }
            self.unlink(last);
            self.link(Loc::end(clone), last);
            last = clone;
        }
        let loc = self.place(Some(common));
        self.unlink(last);
        self.link(loc, last);
        self.adopt_finish(fe, fb, bookmark)
    }
}
