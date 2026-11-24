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

//! Whether a program names GTK's libraries anywhere in its bytes, as a
//! program that links them does in its dynamic section. Pure, so the host
//! proofs hold it to the three-scan rule it replaced.
//!
//! Each name begins "libg", so the program is passed over once and each
//! name compared only where that prefix stands. The three scans this
//! replaced passed over the whole program once per name: 11.9 ms against
//! 3.2 ms for a 7.3 MB program on the host, the same answer either way.

/// The libraries whose presence means GTK's caches are wanted.
pub const LINKS: [&[u8]; 3] = [b"libgtk-3.so", b"libgtk-4.so", b"libgdk_pixbuf-2.0.so"];

const PREFIX: &[u8; 4] = b"libg";

pub fn names_gtk(program: &[u8]) -> bool {
    program.windows(PREFIX.len()).enumerate().any(|(at, w)| {
        w == PREFIX && LINKS.iter().any(|l| program.get(at..).is_some_and(|t| t.starts_with(l)))
    })
}
