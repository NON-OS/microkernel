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

//! The one line each attempt prints, built without an allocator: the attack's
//! name, then what refused it and the errno, or what got through.

use nonos_libc::mk_debug;

use crate::errno::name as errno;

const MAX: usize = 256;

struct Line {
    buf: [u8; MAX],
    len: usize,
}

impl Line {
    fn push(&mut self, bytes: &[u8]) -> &mut Self {
        let n = bytes.len().min(MAX - 1 - self.len);
        self.buf[self.len..self.len + n].copy_from_slice(&bytes[..n]);
        self.len += n;
        self
    }

    fn number(&mut self, mut v: u64) -> &mut Self {
        let mut digits = [0u8; 20];
        let mut i = digits.len();
        loop {
            i -= 1;
            digits[i] = b'0' + (v % 10) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        self.push(&digits[i..])
    }

    fn print(&mut self) {
        self.push(b"\n");
        let _ = mk_debug(self.buf.as_ptr(), self.len);
    }
}

/// The kernel's answer `rc` to `what`: refused on any errno, escaped otherwise.
pub fn verdict(name: &[u8], what: &[u8], rc: i64) {
    let mut l = Line { buf: [0; MAX], len: 0 };
    l.push(b"[ATTACK] ").push(name);
    if rc < 0 {
        l.push(b" refused: ").push(what).push(b" ").push(errno(rc)).push(b" (").number(rc.unsigned_abs()).push(b")");
    } else {
        l.push(b" ESCAPED: ").push(what).push(b" returned ").number(rc as u64);
    }
    l.print();
}

/// A line that is neither: an attempt that could not be made, and why.
pub fn note(text: &[u8]) {
    let _ = mk_debug(text.as_ptr(), text.len());
}

/// A refusal whose evidence is a count the attacker read, not an errno.
pub fn counted(name: &[u8], what: &[u8], count: u64, unit: &[u8]) {
    let mut l = Line { buf: [0; MAX], len: 0 };
    l.push(b"[ATTACK] ").push(name).push(b" refused: ").push(what).push(b" ").number(count).push(b" ").push(unit);
    l.print();
}

/// The line left before an act the attacker cannot report on itself.
pub fn marker(head: &[u8], value: u64, tail: &[u8]) {
    let mut l = Line { buf: [0; MAX], len: 0 };
    l.push(head).number(value).push(tail);
    let _ = mk_debug(l.buf.as_ptr(), l.len);
}

/// What got through to a service that answers in its own status word: the
/// status it gave instead of refusing.
pub fn escaped(name: &[u8], what: &[u8], status: i64) {
    let mut l = Line { buf: [0; MAX], len: 0 };
    l.push(b"[ATTACK] ").push(name).push(b" ESCAPED: ").push(what).push(b", the service answered status ");
    if status < 0 {
        l.push(b"-");
    }
    l.number(status.unsigned_abs());
    l.print();
}
