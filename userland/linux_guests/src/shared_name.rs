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

//! A name two guests could both reach, tried from each side.
//!
//! The holder leaves the pattern under each name; the reader, a family of
//! its own, looks for it. Scratch space is the family's own and the tree is
//! read-only to guests, so the reader must find neither.

use crate::report::{Report, Seen};
use crate::sys::{call, out, CLOSE, OPEN, PATTERN, READ, WRITE};

const O_WRONLY_CREAT: u64 = 0o101;
const NAMES: [&str; 2] = ["/tmp/nonos-sibling\0", "/nonos-sibling\0"];

/// The holder's half: every name written, and what each write returned.
pub fn leave() {
    for name in NAMES {
        let fd = call(OPEN, [name.as_ptr() as u64, O_WRONLY_CREAT, 0o644, 0, 0, 0]);
        let wrote = match fd {
            fd if fd < 0 => fd,
            fd => {
                let n = call(
                    WRITE,
                    [fd as u64, PATTERN.as_ptr() as u64, PATTERN.len() as u64, 0, 0, 0],
                );
                let _ = call(CLOSE, [fd as u64, 0, 0, 0, 0, 0]);
                n
            }
        };
        let shown = name.trim_end_matches('\0');
        out(format!("[GUEST] holder left {shown}: {wrote}\n").as_bytes());
    }
}

/// The reader's half: a name that opens onto the pattern is an escape.
pub fn look(r: &mut Report) {
    for name in NAMES {
        let what = format!("the sibling's {}", name.trim_end_matches('\0'));
        let fd = call(OPEN, [name.as_ptr() as u64, 0, 0, 0, 0, 0]);
        if fd < 0 {
            r.check(&what, Seen::Refused(fd));
            continue;
        }
        let mut buf = [0u8; 16];
        let n = call(READ, [fd as u64, buf.as_mut_ptr() as u64, 16, 0, 0, 0]);
        let _ = call(CLOSE, [fd as u64, 0, 0, 0, 0, 0]);
        let seen = match n == 16 && &buf == PATTERN {
            true => Seen::Escaped("its pattern was there".into()),
            false => Seen::Refused(0),
        };
        r.check(&what, seen);
    }
}
