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

use alloc::rc::Rc;
use alloc::string::ToString;

use crate::browser::js::value::Value;

use super::super::ctx::Ctx;
use super::super::to_str::to_str;

/* getAttribute, setAttribute, removeAttribute and hasAttribute. */
pub(super) fn attr_method(ctx: &mut Ctx, id: usize, method: &str, argv: &[Value]) -> Value {
    let name = argv.first().map(to_str).unwrap_or_default();
    match method {
        "getAttribute" => match ctx.dom.nodes[id].attr(&name) {
            Some(v) => Value::Str(Rc::new(v.to_string())),
            None => Value::Null,
        },
        "setAttribute" => {
            let value = argv.get(1).map(to_str).unwrap_or_default();
            if !name.is_empty() {
                ctx.dom.set_attr(id, &name, value);
                ctx.dirty = true;
            }
            Value::Undef
        }
        "removeAttribute" => {
            if !name.is_empty() {
                ctx.dom.remove_attr(id, &name);
                ctx.dirty = true;
            }
            Value::Undef
        }
        _ => Value::Bool(ctx.dom.nodes[id].attr(&name).is_some()),
    }
}
