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
    /// An a start tag first closes an a still in the list, so links never nest.
    pub(in super::super) fn body_a(&mut self, t: Tag) {
        if let Some(old) = self.fmt_last_named("a") {
            if self.adoption("a") {
                self.any_other_end("a");
            }
            self.remove_fmt(old);
            self.remove_open(old);
        }
        self.open_formatting(t);
    }

    /// A nobr start tag closes an open nobr the way its end tag would.
    pub(in super::super) fn body_nobr(&mut self, t: Tag) {
        self.reconstruct();
        if self.in_scope("nobr", Scope::Default) {
            if self.adoption("nobr") {
                self.any_other_end("nobr");
            }
            self.reconstruct();
        }
        self.open_formatting(t);
    }

    /// Reopen formatting, insert the element and remember it as formatting.
    pub(in super::super) fn open_formatting(&mut self, t: Tag) {
        self.reconstruct();
        if let Some(id) = self.insert_html(t) {
            self.push_fmt(id);
        }
    }
}
