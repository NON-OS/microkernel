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

//! The kept fields as the store holds them now.

use alloc::vec::Vec;

use nonos_policy_proto::field_kind::kind_of;
use nonos_policy_proto::settings_record::{encode, Value, KEPT};
use nonos_policy_proto::{Field, KIND_BOOL, KIND_I8, KIND_STR, KIND_U64, KIND_U8};

use crate::store::types::STRING_CAP;
use crate::store::{get_bool, get_i8, get_str, get_u64, get_u8};

/// The string fields among those kept, read first into buffers of their own
/// so the values can borrow them.
struct Strings {
    fields: Vec<Field>,
    bytes: Vec<([u8; STRING_CAP], usize)>,
}

impl Strings {
    fn read() -> Strings {
        let fields: Vec<Field> = KEPT.iter().copied().filter(|&f| kind_of(f) == KIND_STR).collect();
        let bytes = fields
            .iter()
            .map(|&f| {
                let mut buf = [0u8; STRING_CAP];
                let len = get_str::get(f, &mut buf).unwrap_or(0);
                (buf, len)
            })
            .collect();
        Strings { fields, bytes }
    }

    fn of(&self, field: Field) -> Option<&[u8]> {
        let i = self.fields.iter().position(|&f| f == field)?;
        let (buf, len) = &self.bytes[i];
        Some(&buf[..*len])
    }
}

pub(super) fn record() -> Vec<u8> {
    let strings = Strings::read();
    let values = KEPT.iter().filter_map(|&field| {
        let value = match kind_of(field) {
            KIND_BOOL => get_bool::get(field).map(Value::Bool),
            KIND_U8 => get_u8::get(field).map(Value::U8),
            KIND_I8 => get_i8::get(field).map(Value::I8),
            KIND_U64 => get_u64::get(field).map(Value::U64),
            KIND_STR => strings.of(field).map(Value::Str),
            _ => None,
        };
        value.map(|v| (field, v))
    });
    encode(values)
}
