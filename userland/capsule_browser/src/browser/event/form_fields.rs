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
use alloc::vec::Vec;

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;
use crate::browser::http::urlencode;

use super::field_value::{field_text, select_values};

/// Name and value pairs one submit carries at most.
pub const MAX_FIELDS: usize = 512;

/// A form with more fields than a submit carries. It is not sent at all:
/// one sent with its fields past the limit dropped would be a different
/// request from the one the page built, made without the reader knowing.
#[derive(Debug, PartialEq, Eq)]
pub struct TooManyFields;

/*
 * The form's fields as one urlencoded body, in document order, as HTML
 * builds the form data set: disabled controls, buttons other than the one
 * that submitted, and unchecked checkables stay out. `submitter` is the
 * button or image input that was clicked, None when Enter submitted, and
 * `at` where on an image input it was clicked: it sends name.x and name.y
 * (x and y when it has no name), as HTML builds them.
 *
 * The fields used to be gathered last first, a select sent nothing, a
 * textarea the page filled in was sent empty, the clicked button's own name
 * and value were left out, and past 64 fields the rest were dropped without
 * a word.
 */
pub(super) fn form_fields(
    dom: &Dom,
    form: usize,
    submitter: Option<usize>,
    at: Option<(i32, i32)>,
) -> Result<String, TooManyFields> {
    let mut body = String::new();
    let mut fields = 0usize;
    let mut push = |name: &str, value: &str| {
        if fields == MAX_FIELDS {
            return Err(TooManyFields);
        }
        if !body.is_empty() {
            body.push('&');
        }
        body.push_str(&urlencode(name));
        body.push('=');
        body.push_str(&urlencode(value));
        fields += 1;
        Ok(())
    };
    let mut stack: Vec<usize> = dom.nodes.get(form).map(|n| n.children.clone()).unwrap_or_default();
    stack.reverse();
    while let Some(id) = stack.pop() {
        let Some(n) = dom.nodes.get(id) else {
            continue;
        };
        if n.kind != NodeKind::Element {
            continue;
        }
        stack.extend(n.children.iter().rev().copied());
        let name = n.attr("name").unwrap_or("");
        let image =
            n.tag == "input" && n.attr("type").is_some_and(|t| t.eq_ignore_ascii_case("image"));
        if let (true, Some((x, y))) = (image && submitter == Some(id), at) {
            if n.attr("disabled").is_none() {
                let dot = if name.is_empty() { "" } else { "." };
                push(&alloc::format!("{}{}x", name, dot), &alloc::format!("{}", x))?;
                push(&alloc::format!("{}{}y", name, dot), &alloc::format!("{}", y))?;
            }
            continue;
        }
        if name.is_empty() || n.attr("disabled").is_some() {
            continue;
        }
        match n.tag.as_str() {
            "input" => {
                let ty = n.attr("type").unwrap_or("text").to_ascii_lowercase();
                match ty.as_str() {
                    "submit" if submitter == Some(id) => push(name, n.attr("value").unwrap_or(""))?,
                    "submit" | "button" | "reset" | "image" | "file" => {}
                    "checkbox" | "radio" if n.attr("checked").is_none() => {}
                    "checkbox" | "radio" => push(name, n.attr("value").unwrap_or("on"))?,
                    _ => push(name, &field_text(dom, id))?,
                }
            }
            "button" if submitter == Some(id) => push(name, n.attr("value").unwrap_or(""))?,
            "textarea" => push(name, &field_text(dom, id))?,
            "select" => {
                for value in select_values(dom, id) {
                    push(name, &value)?;
                }
            }
            _ => {}
        }
    }
    Ok(body)
}
