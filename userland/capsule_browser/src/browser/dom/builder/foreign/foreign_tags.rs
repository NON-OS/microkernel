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

use crate::browser::html::tokenizer::{Tag, Token};

use super::super::super::node::Ns;
use super::super::ops::counts::bucket;
use super::super::ops::state::Builder;
use super::adjust::{adjust_mathml, adjust_svg, svg_tag_case};

impl Builder {
    /// A start tag in foreign content: an element in the adjusted current
    /// node's namespace, closed at once when written self-closing, so
    /// `<path/>` does not swallow the shapes after it.
    pub(in super::super) fn foreign_start(&mut self, mut t: Tag) {
        let ns = self.ns(self.adjusted());
        match ns {
            Ns::MathMl => adjust_mathml(&mut t),
            Ns::Svg => {
                svg_tag_case(&mut t.name);
                adjust_svg(&mut t);
            }
            Ns::Html => {}
        }
        let push = !t.self_closing;
        self.insert_element(t, ns, push);
    }

    /// An svg or math start tag in HTML content opens foreign content.
    pub(in super::super) fn foreign_root(&mut self, mut t: Tag, ns: Ns) {
        self.reconstruct();
        match ns {
            Ns::MathMl => adjust_mathml(&mut t),
            _ => adjust_svg(&mut t),
        }
        let push = !t.self_closing;
        self.insert_element(t, ns, push);
    }

    /// An end tag in foreign content closes the nearest foreign element of
    /// that name, or goes to the HTML rules once an HTML element is reached.
    pub(in super::super) fn foreign_end(&mut self, t: Tag) {
        let want = bucket(&t.name) as u16;
        let mut i = self.open.len().saturating_sub(1);
        loop {
            if i == 0 {
                return;
            }
            let hit = self.open_meta[i] & 0xFF == want;
            if hit && self.dom.nodes[self.open[i]].tag.eq_ignore_ascii_case(&t.name) {
                self.open_truncate(i);
                return;
            }
            i -= 1;
            if self.ns(self.open[i]) == Ns::Html {
                self.in_mode(self.mode, Token::End(t));
                return;
            }
        }
    }
}
