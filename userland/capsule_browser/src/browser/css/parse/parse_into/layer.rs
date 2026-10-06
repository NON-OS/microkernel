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

use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::decl::Decl;
use crate::browser::css::rule::Rule;

use super::block::Parent;
use super::cx::Cx;
use super::parse_into;
use super::scan::split_top;

/* @layer name { ... } or an anonymous @layer { ... }: the block's rules
 * sit in that layer, nested in the enclosing one. An anonymous layer is
 * named by its byte offset, which no written name can take. */
pub(super) fn block(
    name: &str,
    head: &str,
    body: &str,
    cx: &mut Cx,
    depth: u32,
    parent: Option<&Parent>,
) {
    let path = if name.is_empty() {
        qualify(cx, &format!("\u{1}{}", cx.at(head)))
    } else {
        qualify(cx, name)
    };
    let outer = cx.layer.replace(Rc::from(path.as_str()));
    parse_into(body, cx, depth + 1, parent);
    cx.layer = outer;
}

/* '@layer a, b.c;' fixes the order of the layers it names, before any
 * of their rules appear: a data rule with one decl per layer path. */
pub(super) fn statement(names: &str, cx: &mut Cx) {
    let decls: Vec<Decl> = split_top(names, b',')
        .filter(|n| !n.is_empty())
        .map(|n| Decl::new(String::from("layer"), qualify(cx, n), false))
        .collect();
    if !decls.is_empty() {
        cx.statement(decls, Rule::LAYER_ORDER);
    }
}

fn qualify(cx: &Cx, name: &str) -> String {
    match &cx.layer {
        Some(outer) => format!("{outer}.{}", name.trim()),
        None => String::from(name.trim()),
    }
}
