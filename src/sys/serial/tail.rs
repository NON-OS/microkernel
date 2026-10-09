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

//! The last 64 KiB written to the serial console, kept in memory for a
//! machine with no serial port to show it (`MkLogTail`), on the images that
//! let capsules write to the console (`capsule-serial-debug`), never on a
//! hardened or air-gapped one. Every byte the
//! console takes is kept here first, whether or not a UART answers. A byte
//! arriving while the tail is being read is dropped from the tail, never
//! waited for: the writer can be an interrupt or a fault.

use spin::Mutex;

pub const CAPACITY: usize = 64 * 1024;
/// The boot's first bytes, kept for good: drivers say how they came up once,
/// early, and a long boot's later lines must not push that out of the tail.
pub const HEAD: usize = 64 * 1024;
/// What `latest` can hand back: the head, then the tail.
pub const KEPT: usize = HEAD + CAPACITY;

struct Head {
    bytes: [u8; HEAD],
    len: usize,
}

static FIRST: Mutex<Head> = Mutex::new(Head { bytes: [0; HEAD], len: 0 });

struct Tail {
    bytes: [u8; CAPACITY],
    next: usize,
    wrapped: bool,
}

static TAIL: Mutex<Tail> = Mutex::new(Tail { bytes: [0; CAPACITY], next: 0, wrapped: false });

/// A hardened image keeps nothing: it is built without capsule serial output,
/// and its console's lines stay on the console.
pub(super) fn keep(b: u8) {
    if !cfg!(feature = "capsule-serial-debug") {
        return;
    }
    if let Some(mut h) = FIRST.try_lock() {
        if h.len < HEAD {
            let at = h.len;
            h.bytes[at] = b;
            h.len = at + 1;
            return;
        }
    }
    if let Some(mut t) = TAIL.try_lock() {
        let at = t.next;
        t.bytes[at] = b;
        t.next = (at + 1) % CAPACITY;
        if t.next == 0 {
            t.wrapped = true;
        }
    }
}

/// The boot's first bytes, then the latest ones, as many as fit in `out`,
/// oldest first; how many. The tail gets the room the head leaves.
pub fn latest(out: &mut [u8]) -> usize {
    crate::arch::run_without_interrupts(|| {
        let head = {
            let h = FIRST.lock();
            let n = h.len.min(out.len());
            out[..n].copy_from_slice(&h.bytes[..n]);
            n
        };
        head + tail_into(&mut out[head..])
    })
}

fn tail_into(out: &mut [u8]) -> usize {
    {
        let t = TAIL.lock();
        let held = if t.wrapped { CAPACITY } else { t.next };
        let n = held.min(out.len());
        let start = (t.next + CAPACITY - n) % CAPACITY;
        let first = n.min(CAPACITY - start);
        out[..first].copy_from_slice(&t.bytes[start..start + first]);
        out[first..n].copy_from_slice(&t.bytes[..n - first]);
        n
    }
}
