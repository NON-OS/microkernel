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

//! Putting back what Settings changed after setup (settings_record).

use nonos_app_skeleton::clients::vfs;
use nonos_policy_proto::settings_record::{decode_values, Value, VALUES_MAX, VALUES_PATH};

use crate::push;
use crate::store::{set_bool, set_i8, set_str, set_u64, set_u8};

pub(super) fn restore(pid: u32) {
    let Ok(raw) = vfs::read_file(pid, VALUES_PATH, VALUES_MAX as u32) else {
        return;
    };
    let Some(values) = decode_values(&raw) else {
        return super::apply::say(b"[POLICY] kept settings refused: the record does not frame\n");
    };
    // Each value goes through the setter Settings' own change went through,
    // so one the store would refuse now is refused here too.
    for (field, value) in values {
        match value {
            Value::Bool(v) if set_bool::set(field, v) => push::on_bool_set(field, v),
            Value::I8(v) if set_i8::set(field, v) => push::on_i8_set(field, v),
            Value::Str(v) if set_str::set(field, v) => push::on_string_set(field, v),
            Value::U8(v) => {
                let _ = set_u8::set(field, v);
            }
            Value::U64(v) => {
                let _ = set_u64::set(field, v);
            }
            _ => {}
        }
    }
    super::apply::say(b"[POLICY] restored what Settings kept on an earlier boot\n");
}
