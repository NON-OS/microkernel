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

use crate::browser::css::calc::viewport;

use super::super::media_matches::media_matches;
use super::block::Parent;
use super::container::container_matches;
use super::cx::Cx;
use super::supports::supports;
use super::verdict::verdict;
use super::{layer, parse_into, property, scope, MAX_DEPTH};

/* A block at-rule. Conditional groups (@media, @supports, @container)
 * whose condition holds read their block as if it stood in their place,
 * inside a style rule too; @layer and @scope read theirs in a layer or
 * scope; @property registers a custom property. Every other block
 * (@font-face, @keyframes, @page, @starting-style, ...) styles no
 * element and is passed over. */
pub(super) fn at_rule(head: &str, body: &str, cx: &mut Cx, depth: u32, parent: Option<&Parent>) {
    if depth >= MAX_DEPTH {
        return;
    }
    let (name, cond) = at_name(head);
    let holds = match name.as_str() {
        "media" => verdict(cx, "media", cond, media_matches(cond, viewport::width())),
        "supports" => supports(cond),
        "container" => verdict(cx, "container", cond, container_matches(cond)),
        "layer" => return layer::block(cond, head, body, cx, depth, parent),
        "property" => return property::register(cond, body, cx),
        "scope" => return scope::scope(cond, body, cx, depth, parent),
        _ => false,
    };
    if holds {
        parse_into(body, cx, depth + 1, parent);
    }
}

/* A statement at-rule; only '@layer a, b;' carries anything the cascade
 * reads (@import and @charset have been handled by the fetcher). */
pub(super) fn statement(head: &str, cx: &mut Cx) {
    let (name, rest) = at_name(head);
    if name == "layer" {
        layer::statement(rest, cx);
    }
}

/* The lowercased name after '@' and the prelude that follows it. */
fn at_name(head: &str) -> (String, &str) {
    let h = head.trim_start_matches('@');
    let end = h.find(|c: char| !(c.is_ascii_alphanumeric() || c == '-')).unwrap_or(h.len());
    (h[..end].to_ascii_lowercase(), h[end..].trim())
}
