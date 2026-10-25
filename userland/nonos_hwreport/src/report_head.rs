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

//! A report's first three lines: the header, the build and the posture.

use crate::report::Report;
use crate::sink::Sink;
use crate::HEADER;

/// A build word as reported: letters, digits, '.', '-' and '_', at most 40.
fn word(w: &[u8], out: &mut Sink) {
    let ok = |b: &u8| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_');
    if w.is_empty() || w.len() > 40 || !w.iter().all(ok) {
        out.bytes(b"unknown");
    } else {
        out.bytes(w);
    }
}

impl<'a> Report<'a> {
    pub fn new(buf: &'a mut [u8], commit: &[u8], profile: &[u8], unconfined: bool) -> Self {
        let mut r = Report { out: Sink::new(buf), lines: 0, cut: false };
        r.put(|s| s.bytes(HEADER).bytes(b"\n").full());
        r.put(|s| {
            word(commit, s.bytes(b"build "));
            word(profile, s.bytes(b" "));
            s.bytes(b"\n").full()
        });
        let posture: &[u8] = if unconfined { b"dma-unconfined" } else { b"confined" };
        r.put(|s| s.bytes(b"posture ").bytes(posture).bytes(b"\n").full());
        r
    }
}
