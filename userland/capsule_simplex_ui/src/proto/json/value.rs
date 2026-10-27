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

//! A parsed JSON value and the lookups the chat API readers use.

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    /// A number as written, read on demand: ids are integers and nothing the
    /// window shows needs a float.
    Num(String),
    Str(String),
    Arr(Vec<Value>),
    Obj(Vec<(String, Value)>),
}

static NULL: Value = Value::Null;

impl Value {
    /// The member named `key`, or Null when this is not an object or has none.
    pub fn get(&self, key: &str) -> &Value {
        match self {
            Value::Obj(members) => {
                members.iter().find(|(k, _)| k == key).map(|(_, v)| v).unwrap_or(&NULL)
            }
            _ => &NULL,
        }
    }

    /// The value at a path of member names.
    pub fn at(&self, path: &[&str]) -> &Value {
        path.iter().fold(self, |v, key| v.get(key))
    }

    pub fn str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn int(&self) -> Option<i64> {
        match self {
            Value::Num(n) => n.split(['.', 'e', 'E']).next()?.parse().ok(),
            _ => None,
        }
    }

    pub fn items(&self) -> &[Value] {
        match self {
            Value::Arr(items) => items,
            _ => &[],
        }
    }
}
