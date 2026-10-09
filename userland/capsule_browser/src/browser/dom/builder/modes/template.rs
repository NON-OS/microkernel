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

use super::super::ops::mode::{Entry, Mode};
use super::super::ops::state::Builder;

impl Builder {
    /// A template start tag, from "in head" or anywhere that defers to it.
    /// Its contents stay ordinary children of the element.
    pub(in super::super) fn template_start(&mut self, t: Tag) {
        self.insert_html(t);
        self.fmt.push(Entry::Marker);
        self.frameset_ok = false;
        self.mode = Mode::InTemplate;
        self.tmpl.push(Mode::InTemplate);
    }

    /// A template end tag.
    pub(in super::super) fn template_end(&mut self) {
        if !self.template_open() {
            return;
        }
        self.implied_end_all();
        self.pop_until("template");
        self.clear_to_marker();
        self.tmpl.pop();
        self.reset_mode();
    }
}
