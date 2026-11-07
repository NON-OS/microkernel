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

//! What the server's reads of guest memory came to on this thread, counted
//! as the capsule makes them: one kernel peer call a MiB (guest/mem.rs
//! MAX_SPAN), or one a strided read of rows (guest/mem_rows.rs), the bytes,
//! and the guest pages the kernel walks for them, one translation each.

use std::cell::Cell;

/// The kernel's ceiling on one peer call, as the capsule's guest/mem.rs.
const MAX_SPAN: u64 = 1 << 20;
const PAGE: u64 = 4096;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Reads {
    pub calls: u64,
    pub bytes: u64,
    pub pages: u64,
}

thread_local! {
    static READS: Cell<Reads> = const { Cell::new(Reads { calls: 0, bytes: 0, pages: 0 }) };
}

/// A read of `len` bytes from guest address `at`, counted.
pub fn note(at: u64, len: u64) {
    let mut r = READS.get();
    let mut done = 0;
    while done < len {
        let take = (len - done).min(MAX_SPAN);
        let first = (at + done) / PAGE;
        let last = (at + done + take - 1) / PAGE;
        r.calls += 1;
        r.pages += last - first + 1;
        done += take;
    }
    r.bytes += len;
    READS.set(r);
}

/// One strided call of `count` rows of `row` bytes, `stride` apart, from
/// guest address `at`, counted: the kernel walks each row's pages on its
/// own.
pub fn note_rows(at: u64, row: u64, count: u64, stride: u64) {
    let mut r = READS.get();
    r.calls += 1;
    for i in 0..count {
        let from = at + i * stride;
        r.pages += (from + row - 1) / PAGE - from / PAGE + 1;
    }
    r.bytes += row * count;
    READS.set(r);
}

/// The reads counted since the last call, and the count started again.
pub fn take() -> Reads {
    READS.replace(Reads::default())
}
