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

use nonos_policy_proto::{kind_of, Field, E_NOT_READY, OP_GET};
use nonos_policy_proto::{KIND_BOOL, KIND_I8, KIND_STR, KIND_U64, KIND_U8};

use super::handlers::{get_bool, get_i8, get_str, get_u64, get_u8};
use super::respond;

/// A kept field asked for before the restore has put it back is answered
/// "not ready", never with the default the store holds until then.
pub fn dispatch(pid: u32, field: Field) {
    if crate::restore::awaits_restore(field) {
        respond::err(pid, OP_GET, field as u32, kind_of(field), E_NOT_READY);
        return;
    }
    match kind_of(field) {
        KIND_BOOL => get_bool::handle(pid, field),
        KIND_U8 => get_u8::handle(pid, field),
        KIND_I8 => get_i8::handle(pid, field),
        KIND_STR => get_str::handle(pid, field),
        KIND_U64 => get_u64::handle(pid, field),
        _ => {}
    }
}
