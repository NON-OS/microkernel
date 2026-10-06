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
use super::scope::Scope;
use super::state::Builder;

impl Builder {
    /// Generate implied end tags (13.2.6.3), leaving an element named
    /// `except` open.
    pub(in super::super) fn implied_end(&mut self, except: Option<&str>) {
        while let Some(&id) = self.open.last() {
            let n = &self.dom.nodes[id];
            let implied = matches!(
                n.tag.as_str(),
                "dd" | "dt" | "li" | "optgroup" | "option" | "p" | "rb" | "rp" | "rt" | "rtc"
            );
            if n.ns != Ns::Html || !implied || Some(n.tag.as_str()) == except {
                return;
            }
            self.pop();
        }
    }

    /// Generate all implied end tags thoroughly: table parts too.
    pub(in super::super) fn implied_end_all(&mut self) {
        while let Some(&id) = self.open.last() {
            let thorough = self.is_any(
                id,
                &[
                    "caption", "colgroup", "dd", "dt", "li", "optgroup", "option", "p", "rb", "rp",
                    "rt", "rtc", "tbody", "td", "tfoot", "th", "thead", "tr",
                ],
            );
            if !thorough {
                return;
            }
            self.pop();
        }
    }

    /// Close a p element.
    pub(in super::super) fn close_p(&mut self) {
        self.implied_end(Some("p"));
        self.pop_until("p");
    }

    /// Close a p element if one is in button scope, as block starts do.
    pub(in super::super) fn close_p_in_button_scope(&mut self) {
        if self.in_scope("p", Scope::Button) {
            self.close_p();
        }
    }

    /// Pop until the current node is one of `names` (HTML) or the root html,
    /// the "clear the stack back to a table context" family.
    pub(in super::super) fn clear_back_to(&mut self, names: &[&str]) {
        while self.open.len() > 1 && !self.cur_is_any(names) && !self.cur_is("html") {
            self.pop();
        }
    }
}
