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
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::css;
use crate::browser::js::value::Value;

use super::super::ctx::Ctx;
use super::super::to_str::to_str;

/* element.querySelector and element.querySelectorAll: only the element's
 * own subtree is walked, with :scope as the element, and every match comes
 * back. Taking the first few hundred matches of the whole document and then
 * keeping those inside the element lost every match past them. */
pub(super) fn query_method(ctx: &mut Ctx, id: usize, method: &str, argv: &[Value]) -> Value {
    let sel = argv.first().map(to_str).unwrap_or_default();
    if method == "querySelector" {
        return match css::select_in(ctx.dom, id, &sel, 1).first() {
            Some(&hit) => Value::Node(hit),
            None => Value::Null,
        };
    }
    let hits: Vec<Value> =
        css::select_in(ctx.dom, id, &sel, usize::MAX).into_iter().map(Value::Node).collect();
    Value::Array(Rc::new(RefCell::new(hits)))
}
