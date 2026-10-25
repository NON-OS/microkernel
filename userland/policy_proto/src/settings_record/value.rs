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

use crate::kind::{KIND_BOOL, KIND_I8, KIND_STR, KIND_U64, KIND_U8};

/// One kept value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value<'a> {
    Bool(bool),
    U8(u8),
    I8(i8),
    U64(u64),
    Str(&'a [u8]),
}

impl Value<'_> {
    pub(super) fn kind(&self) -> u8 {
        match self {
            Value::Bool(_) => KIND_BOOL,
            Value::U8(_) => KIND_U8,
            Value::I8(_) => KIND_I8,
            Value::U64(_) => KIND_U64,
            Value::Str(_) => KIND_STR,
        }
    }
}
