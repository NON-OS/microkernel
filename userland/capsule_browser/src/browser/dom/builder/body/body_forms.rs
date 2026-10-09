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

use crate::browser::html::tokenizer::Tag;

use super::super::ops::scope::Scope;
use super::super::ops::state::Builder;

impl Builder {
    /// An input start tag. Inside a select it closes the select first.
    pub(in super::super) fn body_input(&mut self, t: Tag) {
        if self.context_is("select") {
            return;
        }
        if self.in_scope("select", Scope::Default) {
            self.pop_until("select");
        }
        self.reconstruct();
        let hidden = t.attr("type").is_some_and(|v| v.eq_ignore_ascii_case("hidden"));
        self.insert_void(t);
        if !hidden {
            self.frameset_ok = false;
        }
    }

    /// An hr start tag, which may sit in a select between options.
    pub(in super::super) fn body_hr(&mut self, t: Tag) {
        self.close_p_in_button_scope();
        if self.in_scope("select", Scope::Default) {
            self.implied_end(None);
        }
        self.insert_void(t);
        self.frameset_ok = false;
    }

    /// A select start tag. A select inside a select closes the outer one.
    pub(in super::super) fn body_select(&mut self, t: Tag) {
        if self.context_is("select") {
            return;
        }
        if self.in_scope("select", Scope::Default) {
            self.pop_until("select");
            return;
        }
        self.reconstruct();
        self.insert_html(t);
        self.frameset_ok = false;
    }

    /// An option or optgroup: inside a select the implied end tags close the
    /// previous option; elsewhere only an open option closes.
    pub(super) fn body_option(&mut self, t: Tag) {
        if self.in_scope("select", Scope::Default) {
            self.implied_end(if t.name == "option" { Some("optgroup") } else { None });
        } else if self.cur_is("option") {
            self.pop();
        }
        self.reconstruct();
        self.insert_html(t);
    }
}
