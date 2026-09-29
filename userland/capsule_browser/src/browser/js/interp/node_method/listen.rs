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
use super::super::to_str::to_str;

/* addEventListener: an event name of at most 32 bytes and a function, up
 * to 512 listeners per page. */
pub(super) fn add_listener(ctx: &mut Ctx, id: usize, argv: &[Value]) -> Value {
    let event = argv.first().map(to_str).unwrap_or_default();
    if !event.is_empty() && event.len() <= 32 && ctx.listeners.len() < 512 {
        if let Some(cb @ Value::Func(_)) = argv.get(1) {
            ctx.listeners.push((id, event, cb.clone()));
        }
    }
    Value::Undef
}
