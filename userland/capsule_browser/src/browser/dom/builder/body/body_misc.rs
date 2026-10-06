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

use super::super::ops::mode::Mode;
use super::super::ops::scope::Scope;
use super::super::ops::state::Builder;

impl Builder {
    /// A form end tag. Outside templates it closes the form the pointer
    /// names, wherever that is on the stack.
    pub(in super::super) fn body_end_form(&mut self) {
        if self.parsing_template() {
            if self.in_scope("form", Scope::Default) {
                self.implied_end(None);
                self.pop_until("form");
            }
            return;
        }
        let Some(node) = self.form.take() else {
            return;
        };
        if !self.node_in_scope(node) {
            return;
        }
        self.implied_end(None);
        self.remove_open(node);
    }

    /// A frameset start tag replaces the body while nothing made it a body page.
    pub(in super::super) fn body_frameset(&mut self, t: Tag) {
        let Some(&body) = self.open.get(1) else {
            return;
        };
        if !self.is(body, "body") || !self.frameset_ok {
            return;
        }
        self.unlink(body);
        self.open_truncate(1);
        self.insert_html(t);
        self.mode = Mode::InFrameset;
    }

    /// Any other end tag.
    pub(in super::super) fn any_other_end(&mut self, name: &str) {
        if !self.maybe_open(name) {
            return;
        }
        for i in (0..self.open.len()).rev() {
            let id = self.open[i];
            if self.is(id, name) {
                self.implied_end(Some(name));
                self.open_truncate(i);
                return;
            }
            if self.special_at(i) {
                return;
            }
        }
    }
}
