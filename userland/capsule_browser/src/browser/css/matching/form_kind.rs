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

use crate::browser::dom::node::Node;

/* Input types whose value is typed as text, so readonly applies to them
 * and they are :read-write when editable. */
const TYPED: [&str; 12] = [
    "text",
    "search",
    "url",
    "tel",
    "email",
    "password",
    "date",
    "month",
    "week",
    "time",
    "datetime-local",
    "number",
];

/* The rest of the input types HTML 4.10.5 defines. */
const OTHER: [&str; 10] =
    ["hidden", "range", "color", "checkbox", "radio", "file", "submit", "image", "reset", "button"];

/* The elements :enabled and :disabled apply to. */
pub(super) fn is_control(tag: &str) -> bool {
    matches!(tag, "button" | "input" | "select" | "textarea" | "optgroup" | "option" | "fieldset")
}

/* An input's type keyword, lower-cased. A missing or unknown type is the
 * text type, as HTML defines its default. */
pub(super) fn input_kind(n: &Node) -> &'static str {
    let t = n.attr("type").unwrap_or("text");
    let known = TYPED.iter().chain(OTHER.iter()).copied().find(|k| k.eq_ignore_ascii_case(t));
    known.unwrap_or("text")
}

pub(super) fn typed(kind: &str) -> bool {
    TYPED.contains(&kind)
}

/* A submit button: a button whose type is submit or missing or unknown, or
 * an input of type submit or image. */
pub(super) fn is_submit(n: &Node) -> bool {
    match n.tag.as_str() {
        "button" => n
            .attr("type")
            .is_none_or(|t| !t.eq_ignore_ascii_case("button") && !t.eq_ignore_ascii_case("reset")),
        "input" => matches!(input_kind(n), "submit" | "image"),
        _ => false,
    }
}
