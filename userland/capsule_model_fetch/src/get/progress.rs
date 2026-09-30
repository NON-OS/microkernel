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

/* One line per file, rewritten as it comes: how much, and how fast. */

use alloc::format;

use nonos_libc::mk_uptime_ms;

use crate::out::{over, say, size};

/* A step of the line at least this far apart, and at most a hundred a file. */
const STEP: u64 = 8 << 20;

pub struct Progress<'a> {
    name: &'a str,
    bytes: u64,
    shown: u64,
    from: u64,
    since_ms: i64,
}

impl<'a> Progress<'a> {
    pub fn new(name: &'a str, bytes: u64, from: u64) -> Self {
        Progress { name, bytes, shown: from, from, since_ms: mk_uptime_ms() }
    }

    pub fn at(&mut self, at: u64) {
        if at - self.shown < STEP.max(self.bytes / 100) && at != self.bytes {
            return;
        }
        self.shown = at;
        let secs = (mk_uptime_ms() - self.since_ms).max(1) as u64;
        let rate = (at - self.from) * 1000 / secs;
        let pct = at * 100 / self.bytes.max(1);
        over(&format!(
            "  {}: {pct}%, {} of {}, {}/s",
            self.name,
            size(at),
            size(self.bytes),
            size(rate)
        ));
    }

    /* A line of its own under the progress, which goes on below it. */
    pub fn note(&self, text: &str) {
        over(text);
        say("");
    }

    pub fn end(&self) {
        say("");
    }
}
