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

use super::form_kind::input_kind;

/* :required: an input, select or textarea with the required attribute,
 * for an input type that takes it. */
pub(super) fn required(n: &Node) -> bool {
    n.attr("required").is_some()
        && match n.tag.as_str() {
            "input" => !matches!(
                input_kind(n),
                "hidden" | "range" | "color" | "submit" | "image" | "reset" | "button"
            ),
            "select" | "textarea" => true,
            _ => false,
        }
}

/* :optional: the other inputs, selects and textareas, and (as Chromium
 * counts them) buttons. */
pub(super) fn optional(n: &Node) -> bool {
    match n.tag.as_str() {
        "input" | "select" | "textarea" => !required(n),
        "button" => true,
        _ => false,
    }
}
