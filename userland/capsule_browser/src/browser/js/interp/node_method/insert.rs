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

/* appendChild and insertBefore. */
pub(super) fn insert_method(ctx: &mut Ctx, id: usize, method: &str, argv: &[Value]) -> Value {
    let Some(Value::Node(child)) = argv.first() else {
        return Value::Undef;
    };
    let before = match (method, argv.get(1)) {
        /* A framework reorders by inserting ahead of a sibling. Appending can
         * only build a list once; this is what keeps it correct after that. */
        ("insertBefore", Some(Value::Node(r))) => *r,
        /* A null reference means append, which is what the caller asks for
         * when it is adding at the end. */
        _ => usize::MAX,
    };
    if ctx.dom.place(id, *child, before) {
        ctx.dirty = true;
        return Value::Node(*child);
    }
    Value::Undef
}
