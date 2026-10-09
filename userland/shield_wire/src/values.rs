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

//! Named values, one `name=value` to a line, and a request's fields.
//!
//! A value never holds a newline: the writer replaces one with a space, so a
//! sentence from the shield cannot forge a line the wallet would read as
//! another value.

use alloc::string::String;

/// A reply's values as they are written.
#[derive(Default)]
pub struct Values {
    text: String,
}

impl Values {
    pub fn new() -> Values {
        Values { text: String::new() }
    }

    pub fn put(&mut self, name: &str, value: &str) -> &mut Values {
        self.text.push_str(name);
        self.text.push('=');
        for c in value.chars() {
            self.text.push(if c == '\n' || c == '\r' { ' ' } else { c });
        }
        self.text.push('\n');
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// The newest of `items` (the last ones) that fit in `room` more bytes,
    /// each as a `name` value, in their own order. Returns how many of the
    /// oldest were left out, so a reply never runs past the wallet's buffer.
    pub fn put_newest<S: AsRef<str>>(&mut self, name: &str, items: &[S], room: usize) -> usize {
        let mut used = 0usize;
        let mut first = items.len();
        for item in items.iter().rev() {
            let line = name.len() + item.as_ref().len() + 2;
            if used + line > room {
                break;
            }
            used += line;
            first -= 1;
        }
        for item in items.get(first..).unwrap_or(&[]) {
            self.put(name, item.as_ref());
        }
        first
    }

    /// A finished job's own values after the envelope's. One named as an
    /// envelope value (`job`, `state`) goes as `job_<name>`: a value is
    /// read by its first line, so the envelope's would hide it.
    pub fn put_job(&mut self, text: &str) -> &mut Values {
        for line in text.lines() {
            if let Some((k, v)) = line.split_once('=') {
                match k {
                    "state" => self.put("job_state", v),
                    "job" => self.put("job_job", v),
                    _ => self.put(k, v),
                };
            }
        }
        self
    }
}

/// The value named `name` in a reply's body, the first if it repeats.
pub fn field<'a>(body: &'a str, name: &str) -> Option<&'a str> {
    body.lines().find_map(|line| {
        let (k, v) = line.split_once('=')?;
        (k == name).then_some(v)
    })
}

/// Every value named `name`, in order: a list such as one per balance.
pub fn fields<'a>(body: &'a str, name: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    body.lines().filter_map(move |line| {
        let (k, v) = line.split_once('=')?;
        (k == name).then_some(v)
    })
}
