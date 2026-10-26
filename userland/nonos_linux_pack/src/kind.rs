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

//! What an entry is: a file, executable or not, a directory, or a link.

use crate::refused::Refused;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    File,
    Dir,
    Link,
}

/// The kind byte and the mode byte. Only a file is executable or not; a
/// directory and a link carry 0.
pub fn kind(kind: u8, mode: u8) -> Result<(Kind, bool), Refused> {
    let kind = match kind {
        0 => Kind::File,
        1 => Kind::Dir,
        2 => Kind::Link,
        _ => return Err(Refused::Kind),
    };
    match (mode, kind) {
        (0, _) => Ok((kind, false)),
        (1, Kind::File) => Ok((kind, true)),
        _ => Err(Refused::Kind),
    }
}

/// The two bytes `kind` reads.
pub fn bytes(kind: Kind, exec: bool) -> [u8; 2] {
    let k = match kind {
        Kind::File => 0,
        Kind::Dir => 1,
        Kind::Link => 2,
    };
    [k, u8::from(exec && kind == Kind::File)]
}
