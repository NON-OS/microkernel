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

//! Plain opens of the outside file, one per path shape.

use crate::fs_probe::{escaped, p, OUTSIDE, O_RDONLY};
use crate::report::{Report, Seen};
use crate::sys::{call, OPEN};

pub fn opens(r: &mut Report, up: &str) {
    for (what, path) in [
        ("absolute", format!("/{OUTSIDE}\0")),
        ("dot-dot from root", format!("/{up}{OUTSIDE}\0")),
        ("dot-dot relative", format!("{up}{OUTSIDE}\0")),
        ("dot-dot inside a path", format!("/bin/{up}{OUTSIDE}\0")),
        // The personality reads C strings: whatever follows the NUL must not
        // be what gets opened.
        ("embedded NUL", format!("/nonexistent\0/{up}{OUTSIDE}\0")),
    ] {
        r.check(what, opened(call(OPEN, [p(&path), O_RDONLY, 0, 0, 0, 0])));
    }
}

/// An errno is a refusal; a descriptor or a zero is the call going through.
pub fn opened(rc: i64) -> Seen {
    match rc {
        rc if rc < 0 => Seen::Refused(rc),
        rc => escaped("call", rc),
    }
}
