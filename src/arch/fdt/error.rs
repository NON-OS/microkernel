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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FdtError {
    NullPointer,
    /// The blob does not start on the 8-byte boundary the spec requires.
    Misaligned,
    BadMagic,
    BadVersion,
    OutOfBounds,
    /// `totalsize` is past `MAX_BLOB` or past the bytes actually supplied.
    TooLarge,
    /// A block offset breaks the alignment its contents need.
    BadLayout,
    UnalignedToken,
    UnknownToken,
    StringMissing,
    TruncatedNodeName,
    TruncatedProperty,
    TotalSizeTooSmall,
    /// Nesting past `MAX_DEPTH`.
    TooDeep,
    /// An END_NODE with no node open, or FDT_END with one still open.
    Unbalanced,
    /// The structure block ran out before FDT_END.
    MissingEnd,
    /// Something other than one root node: a second root, a property outside
    /// any node, or a root with a name.
    BadRoot,
    /// A property after a subnode in the same node, which the spec forbids
    /// and which would hide it from every cell-size lookup.
    PropertyAfterNode,
    /// A cell count of zero where one is needed, or past `MAX_CELLS`.
    BadCells,
    /// A string property without its terminator, or with one inside it.
    BadString,
    /// A `reg` address that no `ranges` entry above it maps to the CPU.
    NotTranslatable,
    /// A phandle, path or alias that names no node.
    NoSuchNode,
}
