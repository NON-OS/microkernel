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

//! A recursive descent reader for one JSON text. The core's frames are small
//! and trusted only as data, so nesting is bounded and any fault is None.

use super::string::string;
use super::Value;

/// Deeper than any frame the chat API sends.
const MAX_DEPTH: u32 = 32;

pub struct Reader<'a> {
    pub bytes: &'a [u8],
    pub pos: usize,
}

impl Reader<'_> {
    pub fn skip_ws(&mut self) {
        while matches!(self.bytes.get(self.pos), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    pub fn eat(&mut self, b: u8) -> bool {
        self.skip_ws();
        let hit = self.bytes.get(self.pos) == Some(&b);
        self.pos += hit as usize;
        hit
    }

    fn word(&mut self, w: &[u8], v: Value) -> Option<Value> {
        let hit = self.bytes.get(self.pos..self.pos + w.len())? == w;
        self.pos += w.len();
        hit.then_some(v)
    }

    pub fn value(&mut self, depth: u32) -> Option<Value> {
        if depth > MAX_DEPTH {
            return None;
        }
        self.skip_ws();
        match *self.bytes.get(self.pos)? {
            b'{' => self.object(depth),
            b'[' => self.array(depth),
            b'"' => string(self).map(Value::Str),
            b't' => self.word(b"true", Value::Bool(true)),
            b'f' => self.word(b"false", Value::Bool(false)),
            b'n' => self.word(b"null", Value::Null),
            _ => self.number(),
        }
    }
}

/// The value of a whole text, with nothing but white space after it.
pub fn parse(text: &str) -> Option<Value> {
    let mut r = Reader { bytes: text.as_bytes(), pos: 0 };
    let v = r.value(0)?;
    r.skip_ws();
    (r.pos == r.bytes.len()).then_some(v)
}
