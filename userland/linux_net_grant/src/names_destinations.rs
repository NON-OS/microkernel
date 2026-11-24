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

//! The items of a Terminal run's box, from its argv: argv[0], the report's
//! `--box-key=` word if named, then `--box` and its word. The rule that was
//! the kernel's (src/userspace/tool_capsules/net_box.rs), out of ring 0.

/// The `allow:` items of the box `argv` (NUL-separated) names first, or None.
/// A `--box` anywhere later is the program's own argument and names nothing.
pub fn box_word(argv: &[u8]) -> Option<&[u8]> {
    let mut words = argv.split(|b| *b == 0).skip(1);
    let mut first = words.next();
    if first.is_some_and(|w| w.starts_with(b"--box-key=")) {
        first = words.next();
    }
    if first != Some(b"--box".as_slice()) {
        return None;
    }
    words.next()?.strip_prefix(b"allow:")
}

/// Whether `argv` names a box with at least one `net:` item.
pub fn names_destinations(argv: &[u8]) -> bool {
    box_word(argv).is_some_and(|items| items.split(|c| *c == b',').any(|i| i.starts_with(b"net:")))
}
