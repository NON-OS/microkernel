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

//! A count of things and a length of time, in words.

use super::writer::Writer;

/// `n of m`, written into `out`.
pub fn of(n: u64, m: u64, out: &mut [u8; 48]) -> &[u8] {
    let mut w = Writer { out, len: 0 };
    w.num(n);
    w.text(b" of ");
    w.num(m);
    w.done()
}

/// A duration in milliseconds as the coarsest honest unit: "45 s", "12 min",
/// "3 h 20 min", "2 d 4 h".
pub fn duration(ms: u64, out: &mut [u8; 48]) -> &[u8] {
    let s = ms / 1_000;
    let mut w = Writer { out, len: 0 };
    if s < 60 {
        w.num(s);
        w.text(b" s");
    } else if s < 3_600 {
        w.num(s / 60);
        w.text(b" min");
    } else if s < 86_400 {
        w.num(s / 3_600);
        w.text(b" h ");
        w.num((s % 3_600) / 60);
        w.text(b" min");
    } else {
        w.num(s / 86_400);
        w.text(b" d ");
        w.num((s % 86_400) / 3_600);
        w.text(b" h");
    }
    w.done()
}
