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

use alloc::borrow::Cow;

use crate::browser::html::tokenizer::{Tag, Token};

use super::super::super::quirks::Quirks;
use super::super::ops::mode::{Entry, Mode};
use super::super::ops::state::Builder;

impl Builder {
    /// Start tags for formatting, tables and void elements.
    pub(in super::super) fn body_start_inline(&mut self, mut t: Tag) {
        match &*t.name {
            "a" => self.body_a(t),
            "b" | "big" | "code" | "em" | "font" | "i" | "s" | "small" | "strike" | "strong"
            | "tt" | "u" => self.open_formatting(t),
            "nobr" => self.body_nobr(t),
            "applet" | "marquee" | "object" => {
                self.reconstruct();
                self.insert_html(t);
                self.fmt.push(Entry::Marker);
                self.frameset_ok = false;
            }
            "table" => {
                if self.dom.quirks != Quirks::Full {
                    self.close_p_in_button_scope();
                }
                self.insert_html(t);
                self.frameset_ok = false;
                self.mode = Mode::InTable;
            }
            "area" | "br" | "embed" | "img" | "keygen" | "wbr" => {
                self.reconstruct();
                self.insert_void(t);
                self.frameset_ok = false;
            }
            "input" => self.body_input(t),
            "param" | "source" | "track" => {
                self.insert_void(t);
            }
            "hr" => self.body_hr(t),
            "image" => {
                t.name = Cow::Borrowed("img");
                self.process(Token::Start(t));
            }
            _ => self.body_start_rest(t),
        }
    }
}
