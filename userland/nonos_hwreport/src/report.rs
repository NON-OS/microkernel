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

//! A report, line by line. Each line is made whole in a scratch buffer and
//! goes in only if it fits with room left for the closing `end` line, so a
//! report that ran out of room still parses, and says it was cut.

use crate::pci::Function;
use crate::redact::{scrub, Scrubbed};
use crate::sink::Sink;
use crate::{acpi, pci, MAX_LINE};

/// Room kept for `end <count> cut`.
const END_ROOM: usize = 32;

pub struct Report<'a> {
    pub(crate) out: Sink<'a>,
    pub(crate) lines: u64,
    pub(crate) cut: bool,
}

impl<'a> Report<'a> {
    pub(crate) fn put(&mut self, write: impl FnOnce(&mut Sink) -> bool) {
        let mut scratch = [0u8; MAX_LINE + 1];
        let mut line = Sink::new(&mut scratch);
        let overflow = write(&mut line);
        if overflow || line.is_empty() || self.cut || line.len() + END_ROOM > self.out.room() {
            self.cut |= !line.is_empty();
            return;
        }
        self.out.bytes(line.written());
        self.lines += 1;
    }

    pub fn pci(&mut self, f: &Function) {
        self.put(|s| pci::line(f, s));
    }

    /// A 44-byte MkAcpiMeasured record; one of the wrong size is left out.
    pub fn acpi(&mut self, record: &[u8]) {
        self.put(|s| !acpi::line(record, s) || s.full());
    }

    /// A boot log line, scrubbed; a dropped one is reported as `log -`.
    pub fn log(&mut self, text: &[u8]) {
        self.put(|s| {
            s.bytes(b"log ");
            if scrub(text, MAX_LINE - 5, s) == Scrubbed::Dropped {
                s.bytes(b"-");
            }
            s.bytes(b"\n").full()
        });
    }

    /// Close the report: its length, and whether lines were left out for room.
    pub fn finish(mut self) -> (usize, bool) {
        self.out.bytes(b"end ").dec(self.lines);
        self.out.bytes(if self.cut { b" cut\n" } else { b"\n" });
        (self.out.len(), self.cut)
    }
}
