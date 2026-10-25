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

//! A boot log line made safe to carry off the machine.
//!
//! Two passes. A line that names something personal or unique to this unit
//! (a network name, a serial, a UUID, a user) is dropped whole, since masking
//! one value could leave a fragment of another. Every other line keeps its
//! words, with each value shaped like a MAC, an IP address, a UUID or a long
//! hex or decimal run masked, and any byte outside printable ASCII replaced.

use crate::redact_shapes::value_at;
use crate::redact_words::names_private;
use crate::sink::Sink;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scrubbed {
    Kept,
    Dropped,
}

/// The line, scrubbed, written to `out` and cut at `max` bytes; nothing is
/// written for a dropped line. A value is looked for at every byte, so one
/// glued to a word ("ip10.0.2.15") is masked as surely as one standing alone.
pub fn scrub(line: &[u8], max: usize, out: &mut Sink) -> Scrubbed {
    if names_private(line) {
        return Scrubbed::Dropped;
    }
    let start = out.len();
    let mut i = 0;
    while i < line.len() && out.len() - start < max {
        let n = value_at(line, i);
        if n > 0 {
            out.bytes(b"<x>");
            i += n;
            continue;
        }
        let b = line[i];
        out.bytes(&[if (0x20..0x7f).contains(&b) { b } else { b'?' }]);
        i += 1;
    }
    Scrubbed::Kept
}
