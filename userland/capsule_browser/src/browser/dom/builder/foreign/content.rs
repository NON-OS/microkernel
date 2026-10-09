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

use super::super::super::node::Ns;
use super::super::ops::chars::all_space;
use super::super::ops::state::Builder;

impl Builder {
    /// Tokens in foreign content (13.2.6.5): inside svg or math, away from
    /// an integration point.
    pub(in super::super) fn foreign(&mut self, token: Token) {
        match token {
            Token::Null => self.insert_text(Cow::Borrowed("\u{FFFD}")),
            Token::Chars(s) => {
                if !all_space(&s) {
                    self.frameset_ok = false;
                }
                self.insert_text(s);
            }
            Token::Comment | Token::Doctype(_) | Token::Eof => {}
            Token::Start(t) if breaks_out(&t) => self.break_out(Token::Start(t)),
            Token::End(t) if t.name == "br" || t.name == "p" => self.break_out(Token::End(t)),
            Token::Start(t) => self.foreign_start(t),
            Token::End(t) => self.foreign_end(t),
        }
    }

    /// An HTML tag that cannot be foreign content closes the svg or math
    /// around it and is handled as HTML: this is what keeps an unclosed
    /// <svg> sprite from swallowing the rest of a page.
    fn break_out(&mut self, token: Token) {
        while self.open.len() > 1 {
            let cur = self.cur();
            if self.ns(cur) == Ns::Html || self.mathml_text_point(cur) || self.html_point(cur) {
                break;
            }
            self.pop();
        }
        self.in_mode(self.mode, token);
    }
}

/// The HTML start tags that end foreign content (13.2.6.5).
fn breaks_out(t: &Tag) -> bool {
    let font = t.name == "font" && ["color", "face", "size"].iter().any(|a| t.attr(a).is_some());
    font || breaking_name(&t.name)
}

fn breaking_name(name: &str) -> bool {
    matches!(name, "b" | "big" | "blockquote" | "body" | "br" | "center" | "code" | "dd" | "div")
        || matches!(name, "dl" | "dt" | "em" | "embed" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6")
        || matches!(name, "head" | "hr" | "i" | "img" | "li" | "listing" | "menu" | "meta" | "nobr")
        || matches!(name, "ol" | "p" | "pre" | "ruby" | "s" | "small" | "span" | "strong")
        || matches!(name, "strike" | "sub" | "sup" | "table" | "tt" | "u" | "ul" | "var")
}
