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

use crate::browser::dom::Dom;

use super::form_disabled::disabled;
use super::form_kind::{input_kind, typed};

/* Parent hops an editing-host lookup climbs; a script can build a loop. */
const MAX_HOPS: u32 = 512;

/* :read-write: a text-typed input or a textarea that is neither readonly
 * nor disabled, or an element inside editable content (the nearest
 * contenteditable at or above it says true, empty or plaintext-only). */
pub(super) fn read_write(dom: &Dom, id: usize) -> bool {
    let Some(n) = dom.nodes.get(id) else { return false };
    match n.tag.as_str() {
        "input" if !typed(input_kind(n)) => false,
        "input" | "textarea" => n.attr("readonly").is_none() && !disabled(dom, id),
        _ => {
            let mut node = id;
            for _ in 0..MAX_HOPS {
                let Some(e) = dom.nodes.get(node).filter(|_| node != 0) else { return false };
                match e.attr("contenteditable").map(str::to_ascii_lowercase).as_deref() {
                    Some("" | "true" | "plaintext-only") => return true,
                    Some("false") => return false,
                    _ => node = e.parent,
                }
            }
            false
        }
    }
}

/* :placeholder-shown: a text-typed input without a value, or a textarea
 * without text, whose placeholder has something besides line breaks. */
pub(super) fn placeholder_shown(dom: &Dom, id: usize) -> bool {
    let Some(n) = dom.nodes.get(id) else { return false };
    let hint = n.attr("placeholder").is_some_and(|p| p.chars().any(|c| c != '\n' && c != '\r'));
    hint && match n.tag.as_str() {
        "input" => {
            matches!(
                input_kind(n),
                "text" | "search" | "url" | "tel" | "email" | "password" | "number"
            ) && n.attr("value").is_none_or(str::is_empty)
        }
        "textarea" => {
            n.children.iter().all(|&c| dom.nodes.get(c).is_none_or(|t| t.text.is_empty()))
        }
        _ => false,
    }
}
