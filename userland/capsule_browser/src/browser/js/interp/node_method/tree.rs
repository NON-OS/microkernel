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

use crate::browser::js::value::Value;

use super::super::ctx::Ctx;
use super::super::in_subtree::in_subtree;

/* replaceChild, removeChild, cloneNode, contains and remove. */
pub(super) fn tree_method(ctx: &mut Ctx, id: usize, method: &str, argv: &[Value]) -> Value {
    match (method, argv.first(), argv.get(1)) {
        ("replaceChild", Some(Value::Node(fresh)), Some(Value::Node(old))) => {
            let (fresh, old) = (*fresh, *old);
            if ctx.dom.nodes.get(old).is_some_and(|n| n.parent == id)
                && ctx.dom.insert_before(id, fresh, old)
            {
                ctx.dom.detach(old);
                ctx.dirty = true;
                return Value::Node(old);
            }
            Value::Undef
        }
        ("removeChild", Some(Value::Node(child)), _) => {
            let child = *child;
            if ctx.dom.nodes.get(child).is_some_and(|n| n.parent == id) {
                ctx.dom.detach(child);
                ctx.dirty = true;
                return Value::Node(child);
            }
            Value::Undef
        }
        /* A page writes a row's shape once in markup and every row is a copy
         * of it. Without this a script has to build each one tag by tag. */
        ("cloneNode", deep, _) => {
            let deep = matches!(deep, Some(Value::Bool(true)));
            match ctx.dom.clone_node(id, deep) {
                Some(copy) => Value::Node(copy),
                None => Value::Undef,
            }
        }
        ("contains", Some(Value::Node(other)), _) => Value::Bool(in_subtree(ctx.dom, id, *other)),
        ("contains", _, _) => Value::Bool(false),
        ("remove", _, _) => {
            ctx.dom.detach(id);
            ctx.dirty = true;
            Value::Undef
        }
        _ => Value::Undef,
    }
}
