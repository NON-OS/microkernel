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

//! Arrays, objects and numbers.

use alloc::string::String;
use alloc::vec::Vec;

use super::parse::Reader;
use super::string::string;
use super::Value;

impl Reader<'_> {
    pub(super) fn number(&mut self) -> Option<Value> {
        let start = self.pos;
        while matches!(
            self.bytes.get(self.pos),
            Some(b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E')
        ) {
            self.pos += 1;
        }
        let text = core::str::from_utf8(&self.bytes[start..self.pos]).ok()?;
        (!text.is_empty()).then(|| Value::Num(String::from(text)))
    }

    pub(super) fn array(&mut self, depth: u32) -> Option<Value> {
        self.pos += 1;
        let mut items = Vec::new();
        if self.eat(b']') {
            return Some(Value::Arr(items));
        }
        loop {
            items.push(self.value(depth + 1)?);
            if self.eat(b']') {
                return Some(Value::Arr(items));
            }
            self.eat(b',').then_some(())?;
        }
    }

    pub(super) fn object(&mut self, depth: u32) -> Option<Value> {
        self.pos += 1;
        let mut members = Vec::new();
        if self.eat(b'}') {
            return Some(Value::Obj(members));
        }
        loop {
            self.skip_ws();
            let key = string(self)?;
            self.eat(b':').then_some(())?;
            members.push((key, self.value(depth + 1)?));
            if self.eat(b'}') {
                return Some(Value::Obj(members));
            }
            self.eat(b',').then_some(())?;
        }
    }
}
