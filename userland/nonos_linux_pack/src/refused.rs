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

//! Why a pack, or a file in one, is refused. Each is whole: nothing of a
//! refused pack is served.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// Not "NONOSPK1", or a version or flags this parser does not read.
    Magic,
    /// The header's offsets are not the layout's, or not the pack's length.
    Layout,
    /// Larger than MAX_PACK, or more than MAX_ENTRIES entries.
    TooLarge,
    /// A name out of the names block, empty, too long, or not a plain
    /// relative path.
    Name,
    /// Two names out of order, or the same name twice.
    Order,
    /// A kind or mode this version does not have.
    Kind,
    /// A file's bytes out of the data area, misaligned, or over another's;
    /// a directory with bytes; a link with no target or too long a one.
    Data,
    /// A file whose bytes are not its digest.
    Digest,
    /// More files than a pack holds, or one too large, for `build`.
    Build,
    /// Too little free memory to lay the pack out, for `build`.
    Memory,
}
