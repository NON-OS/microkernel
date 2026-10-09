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
use super::attrs::attr_method;
use super::insert::insert_method;
use super::listen::add_listener;
use super::query::query_method;
use super::tree::tree_method;

/* Element methods. Unknown methods answer undefined so scripts keep going. */
pub fn node_method(ctx: &mut Ctx, id: usize, method: &str, argv: &[Value]) -> Value {
    if id >= ctx.dom.nodes.len() {
        return Value::Undef;
    }
    match method {
        "getAttribute" | "setAttribute" | "removeAttribute" | "hasAttribute" => {
            attr_method(ctx, id, method, argv)
        }
        "appendChild" | "insertBefore" => insert_method(ctx, id, method, argv),
        "replaceChild" | "removeChild" | "cloneNode" | "contains" | "remove" => {
            tree_method(ctx, id, method, argv)
        }
        "addEventListener" => add_listener(ctx, id, argv),
        "querySelector" | "querySelectorAll" => query_method(ctx, id, method, argv),
        _ => Value::Undef,
    }
}
