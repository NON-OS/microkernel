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

//! Values written in bulk: the newest of a list that fits a reply's room,
//! and a finished job's own values after the envelope's.

use crate::values::Values;

impl Values {
    /// The newest of `items` (the last ones) that fit in `room` more bytes,
    /// each as a `name` value, in their own order. Returns how many of the
    /// oldest were left out, so a reply never runs past the wallet's buffer.
    pub fn put_newest<S: AsRef<str>>(&mut self, name: &str, items: &[S], room: usize) -> usize {
        let mut used = 0usize;
        let mut first = items.len();
        for item in items.iter().rev() {
            let line = name.len().saturating_add(item.as_ref().len()).saturating_add(2);
            if used.saturating_add(line) > room {
                break;
            }
            used = used.saturating_add(line);
            first = first.saturating_sub(1);
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
