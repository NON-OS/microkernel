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

use crate::arch::fdt::FdtError;

/// Why a blob was refused, as a reason for `refuse`.
pub(super) fn describe(error: FdtError) -> &'static [u8] {
    match error {
        FdtError::NullPointer => b"no blob",
        FdtError::Misaligned => b"blob not on an 8-byte boundary",
        FdtError::BadMagic => b"not a flattened device tree (bad magic)",
        FdtError::BadVersion => b"device tree version older than 17",
        FdtError::OutOfBounds => b"a block runs past the blob",
        FdtError::TooLarge => b"blob larger than 2 MiB",
        FdtError::BadLayout => b"a block is misaligned",
        FdtError::UnalignedToken => b"structure token misaligned",
        FdtError::UnknownToken => b"unknown structure token",
        FdtError::StringMissing => b"property name outside the strings block",
        FdtError::TruncatedNodeName => b"node name runs off the structure block",
        FdtError::TruncatedProperty => b"property runs off the structure block",
        FdtError::TotalSizeTooSmall => b"totalsize smaller than the header",
        FdtError::TooDeep => b"nodes nested deeper than 16",
        FdtError::Unbalanced => b"unbalanced node structure",
        FdtError::MissingEnd => b"structure block ends without FDT_END",
        FdtError::BadRoot => b"not exactly one root node",
        FdtError::PropertyAfterNode => b"property after a subnode",
        FdtError::BadCells => b"a cell count or interrupt specifier is unreadable",
        FdtError::BadString => b"a string property is malformed",
        FdtError::NotTranslatable => b"a device address no bus maps to the CPU",
        FdtError::NoSuchNode => b"a phandle, path or alias names no node",
    }
}
