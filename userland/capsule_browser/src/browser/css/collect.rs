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

use alloc::string::String;

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

mod rule_end;
use rule_end::rule_end;

/* The page's CSS text budget, the same one fetched sheets share. */
const MAX_PAGE_CSS: usize = 8 * 1024 * 1024;

/// The text of every inline `<style>` in document order. A `media`
/// attribute wraps the sheet in `@media <list>{...}` so the parser applies
/// it only where the list matches, keeping its place in the cascade; a
/// non-CSS `type` skips it. Past the page budget the text stops at the
/// last complete rule.
pub fn collect_css(dom: &Dom) -> String {
    let mut s = String::new();
    for n in &dom.nodes {
        if n.kind != NodeKind::Element || n.tag != "style" {
            continue;
        }
        let ty = n.attr("type").unwrap_or("").split(';').next().unwrap_or("").trim();
        if !ty.is_empty() && !ty.eq_ignore_ascii_case("text/css") {
            continue;
        }
        let media = n.attr("media").map(str::trim).filter(|m| !m.eq_ignore_ascii_case("all"));
        if media.is_some_and(|m| m.contains(['{', '}', ';'])) {
            continue;
        }
        let mut text = String::new();
        for &c in &n.children {
            if dom.nodes[c].kind == NodeKind::Text {
                text.push_str(&dom.nodes[c].text);
                text.push('\n');
            }
        }
        let (open, close) = match media.filter(|m| !m.is_empty()) {
            Some(m) => (alloc::format!("@media {m}{{"), "}\n"),
            None => (String::new(), ""),
        };
        let room = MAX_PAGE_CSS.saturating_sub(s.len() + open.len() + close.len());
        let fits = text.len() <= room;
        let body = if fits { &text[..] } else { &text[..rule_end(&text, room)] };
        s.push_str(&open);
        s.push_str(body);
        s.push_str(close);
        if !fits {
            break;
        }
    }
    s
}
