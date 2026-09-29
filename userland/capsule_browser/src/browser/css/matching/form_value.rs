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

use super::cx::Cx;
use super::form_group::group_checked;
use super::form_kind::{input_kind, typed};
use super::form_range::{num, range_state, step_mismatch};
use super::form_required::required;
use super::form_shape::{email, url};

/* An input's static constraints: a required value that is missing, an
 * email or url value of the wrong shape, a number or date outside its
 * range, a number off its step. tooLong and tooShort only arise from user
 * edits. A pattern needs a regular-expression engine this matcher does not
 * carry, so an input with a pattern and a value is undecided (None). */
pub(super) fn input_suffering(cx: &Cx, id: usize, n: &Node) -> Option<bool> {
    let kind = input_kind(n);
    let raw = n.attr("value").unwrap_or("");
    /* Value sanitization: surrounding whitespace goes from email and url
     * values, and a number that does not parse becomes empty. */
    let value = match kind {
        "email" | "url" => raw.trim_matches(|c: char| c.is_ascii_whitespace()),
        "number" if num(Some(raw)).is_none() => "",
        _ => raw,
    };
    let missing = required(n)
        && match kind {
            "checkbox" => n.attr("checked").is_none(),
            "radio" => !group_checked(cx, id),
            "file" => true,
            _ => value.is_empty(),
        };
    let shape = !value.is_empty()
        && match kind {
            "email" if n.attr("multiple").is_some() => !value.split(',').all(|v| email(v.trim())),
            "email" => !email(value),
            "url" => !url(value),
            _ => false,
        };
    if missing || shape || range_state(n) == Some(false) || (kind == "number" && step_mismatch(n)) {
        return Some(true);
    }
    (n.attr("pattern").is_none() || value.is_empty() || !typed(kind)).then_some(false)
}
