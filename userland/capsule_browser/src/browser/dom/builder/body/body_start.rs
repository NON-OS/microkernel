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

use crate::browser::html::tokenizer::{Tag, TextMode, Token};

use super::super::ops::state::Builder;

/// The headings, which close each other.
pub const HEADINGS: &[&str] = &["h1", "h2", "h3", "h4", "h5", "h6"];

impl Builder {
    /// A start tag in body content.
    pub(in super::super) fn body_start(&mut self, t: Tag) {
        match &*t.name {
            "html" => self.body_html(t),
            "base" | "basefont" | "bgsound" | "link" | "meta" | "noframes" | "script" | "style"
            | "template" | "title" => self.in_head(Token::Start(t)),
            "body" => self.body_body(t),
            "frameset" => self.body_frameset(t),
            "address" | "article" | "aside" | "blockquote" | "center" | "details" | "dialog"
            | "dir" | "div" | "dl" | "fieldset" | "figcaption" | "figure" | "footer" | "header"
            | "hgroup" | "main" | "menu" | "nav" | "ol" | "p" | "search" | "section"
            | "summary" | "ul" => {
                self.close_p_in_button_scope();
                self.insert_html(t);
            }
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                self.close_p_in_button_scope();
                if self.cur_is_any(HEADINGS) {
                    self.pop();
                }
                self.insert_html(t);
            }
            "pre" | "listing" => {
                self.close_p_in_button_scope();
                self.insert_html(t);
                self.skip_lf = true;
                self.frameset_ok = false;
            }
            "form" => self.body_form(t),
            "li" => self.body_list_item(t, &["li"]),
            "dd" | "dt" => self.body_list_item(t, &["dd", "dt"]),
            "plaintext" => {
                self.close_p_in_button_scope();
                self.insert_html(t);
                self.switch_to = Some(TextMode::Plaintext);
            }
            _ => self.body_start_inline(t),
        }
    }
}
