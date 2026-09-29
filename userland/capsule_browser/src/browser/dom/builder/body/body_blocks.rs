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

use super::super::ops::state::Builder;

impl Builder {
    /// A form start tag: at most one form open outside templates.
    pub(in super::super) fn body_form(&mut self, t: Tag) {
        if self.form.is_some() && !self.parsing_template() {
            return;
        }
        self.close_p_in_button_scope();
        let id = self.insert_html(t);
        if !self.parsing_template() {
            self.form = id;
        }
    }

    /// An li, dd or dt start tag closes the open item of its kind (`kinds`)
    /// unless something special other than address, div and p is between.
    pub(in super::super) fn body_list_item(&mut self, t: Tag, kinds: &[&str]) {
        self.frameset_ok = false;
        for i in (0..self.open.len()).rev() {
            let id = self.open[i];
            if let Some(&kind) = kinds.iter().find(|k| self.is(id, k)) {
                self.implied_end(Some(kind));
                self.pop_until(kind);
                break;
            }
            if self.special_at(i) && !self.is_any(id, &["address", "div", "p"]) {
                break;
            }
        }
        self.close_p_in_button_scope();
        self.insert_html(t);
    }

    /// A second html start tag lends its attributes to the first.
    pub(in super::super) fn body_html(&mut self, t: Tag) {
        if !self.template_open() {
            if let Some(&root) = self.open.first() {
                self.merge_attrs(root, t.attrs);
            }
        }
    }

    /// A second body start tag lends its attributes to the first.
    pub(in super::super) fn body_body(&mut self, t: Tag) {
        let Some(&body) = self.open.get(1) else {
            return;
        };
        if !self.is(body, "body") || self.template_open() {
            return;
        }
        self.frameset_ok = false;
        self.merge_attrs(body, t.attrs);
    }
}
