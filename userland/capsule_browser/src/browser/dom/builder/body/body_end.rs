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

use super::super::ops::kinds::FORMATTING;
use super::super::ops::scope::Scope;
use super::super::ops::state::Builder;
use super::body_start::HEADINGS;
use crate::browser::html::tokenizer::{Tag, Token};

impl Builder {
    /// An end tag in body content; </br> is a br with its attributes dropped.
    pub(in super::super) fn body_end(&mut self, t: Tag) {
        let name = &*t.name;
        match name {
            "template" => self.in_head(Token::End(t)),
            "body" | "html" => self.body_close(Token::End(t)),
            "address" | "article" | "aside" | "blockquote" | "button" | "center" | "details"
            | "dialog" | "dir" | "div" | "dl" | "fieldset" | "figcaption" | "figure" | "footer"
            | "header" | "hgroup" | "listing" | "main" | "menu" | "nav" | "ol" | "pre"
            | "search" | "section" | "select" | "summary" | "ul" => {
                if self.in_scope(name, Scope::Default) {
                    self.implied_end(None);
                    self.pop_until(name);
                }
            }
            "form" => self.body_end_form(),
            "p" => {
                if !self.in_scope("p", Scope::Button) {
                    self.insert_html(Tag::implied("p"));
                }
                self.close_p();
            }
            "li" | "dd" | "dt" => {
                let scope = if name == "li" { Scope::ListItem } else { Scope::Default };
                if self.in_scope(name, scope) {
                    self.implied_end(Some(name));
                    self.pop_until(name);
                }
            }
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                if self.any_in_scope(HEADINGS) {
                    self.implied_end(None);
                    self.pop_until_any(HEADINGS);
                }
            }
            "applet" | "marquee" | "object" => {
                if self.in_scope(name, Scope::Default) {
                    self.implied_end(None);
                    self.pop_until(name);
                    self.clear_to_marker();
                }
            }
            "br" => self.body_start(Tag::implied("br")),
            _ if FORMATTING.contains(&name) => {
                if self.adoption(name) {
                    self.any_other_end(name);
                }
            }
            _ => self.any_other_end(name),
        }
    }
}
