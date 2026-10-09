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

use crate::browser::html::tokenizer::Token;

use super::super::super::node::Ns;
use super::super::ops::state::Builder;

impl Builder {
    /// Whether the token follows the insertion mode's rules rather than
    /// those for foreign content.
    pub(in super::super) fn html_rules(&self, token: &Token) -> bool {
        if self.open.is_empty() || matches!(token, Token::Eof) {
            return true;
        }
        let node = self.adjusted();
        let n = &self.dom.nodes[node];
        if n.ns == Ns::Html {
            return true;
        }
        let start = match token {
            Token::Start(t) => Some(&*t.name),
            _ => None,
        };
        let chars = matches!(token, Token::Chars(_) | Token::Null);
        if self.mathml_text_point(node)
            && (chars || start.is_some_and(|s| s != "mglyph" && s != "malignmark"))
        {
            return true;
        }
        if n.ns == Ns::MathMl && n.tag == "annotation-xml" && start == Some("svg") {
            return true;
        }
        self.html_point(node) && (chars || start.is_some())
    }

    /// Whether `node` is a MathML text integration point.
    pub(in super::super) fn mathml_text_point(&self, node: usize) -> bool {
        let n = &self.dom.nodes[node];
        n.ns == Ns::MathMl && matches!(n.tag.as_str(), "mi" | "mo" | "mn" | "ms" | "mtext")
    }

    /// Whether `node` is an HTML integration point.
    pub(in super::super) fn html_point(&self, node: usize) -> bool {
        let n = &self.dom.nodes[node];
        match n.ns {
            Ns::Svg => matches!(n.tag.as_str(), "foreignObject" | "desc" | "title"),
            Ns::MathMl if n.tag == "annotation-xml" => n.attr("encoding").is_some_and(|e| {
                e.eq_ignore_ascii_case("text/html")
                    || e.eq_ignore_ascii_case("application/xhtml+xml")
            }),
            _ => false,
        }
    }
}
