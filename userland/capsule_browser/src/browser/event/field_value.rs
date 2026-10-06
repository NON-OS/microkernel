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

//! What a form control holds, as the reader sees it and a submit sends it.
//!
//! Typing keeps a field's text in its `value` attribute, and a submit used
//! to read only that. A textarea the page filled in (the text of a comment
//! being edited) has its text as children and no such attribute, so it was
//! sent empty, and the first key typed into it wiped what it showed. A
//! select has no value attribute at all: the option it shows was never what
//! it sent, which was nothing.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::browser::dom::node::{Node, NodeKind};
use crate::browser::dom::Dom;

/// The text of a text field or textarea: what typing left in its value
/// attribute, else for a textarea the text the page wrote in it.
pub fn field_text(dom: &Dom, id: usize) -> String {
    let Some(node) = dom.nodes.get(id) else { return String::new() };
    if let Some(v) = node.attr("value") {
        return v.to_string();
    }
    if node.tag != "textarea" {
        return String::new();
    }
    let mut out = String::new();
    for t in node.children.iter().filter_map(|&c| dom.nodes.get(c)) {
        if t.kind == NodeKind::Text {
            out.push_str(&t.text);
        }
    }
    out
}

/// What a select sends: for one that takes a single choice, the option it
/// shows (the first selected, else the first), and for a `multiple` one
/// every selected option. An option sends its value, else its text.
pub fn select_values(dom: &Dom, id: usize) -> Vec<String> {
    let Some(select) = dom.nodes.get(id) else { return Vec::new() };
    let options: Vec<&Node> = options(dom, select).collect();
    let chosen = options.iter().filter(|o| o.attr("selected").is_some());
    let picked: Vec<&&Node> = if select.attr("multiple").is_some() {
        chosen.collect()
    } else {
        chosen.chain(options.iter()).take(1).collect()
    };
    picked.into_iter().map(|o| option_value(dom, o)).collect()
}

/// A control's `value` as a script reads it: a text field's or textarea's
/// text, the value a select sends (the first, for a `multiple` one), an
/// option's value, else the value attribute.
pub fn control_value(dom: &Dom, id: usize) -> String {
    match dom.nodes.get(id).map(|n| n.tag.as_str()) {
        Some("input" | "textarea") => field_text(dom, id),
        Some("select") => select_values(dom, id).into_iter().next().unwrap_or_default(),
        Some("option") => option_value(dom, &dom.nodes[id]),
        Some(_) => dom.nodes[id].attr("value").unwrap_or("").to_string(),
        None => String::new(),
    }
}

/// A script's `el.value = v`. A select chooses its first option sending
/// `v` and no other; when none sends it the choice stays as it was, where
/// a browser would show no choice at all, which this select cannot draw.
/// Any other control keeps `v` as its value.
pub fn set_control_value(dom: &mut Dom, id: usize, v: &str) {
    let Some(node) = dom.nodes.get(id) else { return };
    if node.tag != "select" {
        dom.set_attr(id, "value", String::from(v));
        return;
    }
    let ids: Vec<usize> = option_ids(dom, id);
    let Some(pick) = ids.iter().copied().find(|&o| option_value(dom, &dom.nodes[o]) == v) else {
        return;
    };
    for o in ids {
        match o == pick {
            true => dom.set_attr(o, "selected", String::new()),
            false => dom.remove_attr(o, "selected"),
        }
    }
}

/// The reader chose `option` of `select` from its list (select_list). A
/// select taking one choice shows it and no other; in a `multiple` one the
/// option turns on or off. Answers whether what the select sends changed,
/// which is when the page hears input and change.
pub fn choose_option(dom: &mut Dom, select: usize, option: usize) -> bool {
    let ids = option_ids(dom, select);
    if !ids.contains(&option) {
        return false;
    }
    if dom.nodes[select].attr("multiple").is_some() {
        match dom.nodes[option].attr("selected").is_some() {
            true => dom.remove_attr(option, "selected"),
            false => dom.set_attr(option, "selected", String::new()),
        }
        return true;
    }
    let shown = ids.iter().copied().find(|&o| dom.nodes[o].attr("selected").is_some());
    let was = shown.or(ids.first().copied());
    for o in ids {
        match o == option {
            true => dom.set_attr(o, "selected", String::new()),
            false => dom.remove_attr(o, "selected"),
        }
    }
    was != Some(option)
}

/* The DOM ids of a select's options, in the order options() walks them. */
fn option_ids(dom: &Dom, select: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for &c in &dom.nodes[select].children {
        let Some(n) = dom.nodes.get(c) else { continue };
        let inner: Vec<usize> = match n.tag.as_str() {
            "optgroup" => n.children.clone(),
            _ => alloc::vec![c],
        };
        out.extend(inner.into_iter().filter(|&o| {
            dom.nodes.get(o).is_some_and(|e| e.kind == NodeKind::Element && e.tag == "option")
        }));
    }
    out
}

/* The <option> elements of a select, directly or inside an <optgroup>, in
 * document order, as layout's field_label walks them. */
fn options<'a>(dom: &'a Dom, select: &'a Node) -> impl Iterator<Item = &'a Node> + 'a {
    let kids = move |n: &'a Node| n.children.iter().filter_map(move |&c| dom.nodes.get(c));
    kids(select)
        .flat_map(move |c| {
            let group = (c.tag == "optgroup").then(|| kids(c)).into_iter().flatten();
            core::iter::once(c).chain(group)
        })
        .filter(|o| o.kind == NodeKind::Element && o.tag == "option")
}

/* An option's value attribute, else its text with white space collapsed. */
fn option_value(dom: &Dom, option: &Node) -> String {
    if let Some(v) = option.attr("value") {
        return v.to_string();
    }
    let mut text = String::new();
    for t in option.children.iter().filter_map(|&c| dom.nodes.get(c)) {
        if t.kind == NodeKind::Text {
            text.push_str(&t.text);
        }
    }
    let words: Vec<&str> = text.split_ascii_whitespace().collect();
    words.join(" ")
}
