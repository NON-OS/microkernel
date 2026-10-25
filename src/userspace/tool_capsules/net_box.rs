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

//! Whether a terminal's `linux` argv names a box with destinations, the one
//! case the personality is started in its role that holds Network. Pure, so
//! a host proof holds it as it ships.

/// The argv blob (NUL-separated words) is the terminal's boxed run: argv[0],
/// the report's `--box-key=` word if named, then `--box` and a box word
/// naming at least one `net:` item. A `--box` anywhere later is the
/// program's own argument and names nothing here.
pub fn names_destinations(argv: &[u8]) -> bool {
    let mut words = argv.split(|b| *b == 0).skip(1);
    let mut first = words.next();
    if first.is_some_and(|w| w.starts_with(b"--box-key=")) {
        first = words.next();
    }
    first == Some(b"--box".as_slice())
        && words.next().and_then(|w| w.strip_prefix(b"allow:")).is_some_and(|items| {
            items.split(|c| *c == b',').any(|i| i.starts_with(b"net:"))
        })
}
