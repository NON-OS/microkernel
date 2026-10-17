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

//! Why a definition block was refused, and where.

/// What was wrong. Every refusal names a cause; nothing is guessed past one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// The bytes ended inside an object.
    Truncated,
    /// A PkgLength with its reserved bits set, or shorter than its own bytes.
    PkgLength,
    /// A package that claims to end beyond the object holding it.
    PastParent,
    /// A NameSeg with a character section 20.2.2 does not allow.
    Name,
    /// A string with a byte outside 0x01-0x7F before its NUL.
    String,
    /// A byte that starts no object in chapter 20's grammar.
    Opcode(u16),
    /// A known opcode where the grammar does not allow it.
    Misplaced(u16),
    /// Nesting deeper than the arena's limit.
    Depth,
    /// More objects than the arena's limit.
    Nodes,
    /// A table larger than the arena's limit.
    Size,
    /// A table header whose length field disagrees with the table.
    Header,
    /// A table whose bytes do not sum to zero (section 5.2.6).
    Checksum,
    /// A table out of order: the DSDT comes first, then SSDTs and PSDTs.
    Signature,
    /// The evaluation loading a table ran out of budget in its load-time
    /// code; the table was taken back out.
    Budget,
}

/// A refusal and the table offset it was found at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
    pub at: u32,
}

impl Error {
    pub const fn new(kind: ErrorKind, at: u32) -> Self {
        Self { kind, at }
    }
}
