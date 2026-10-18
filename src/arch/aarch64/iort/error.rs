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

/// Why an IORT was refused. Each is a table that would route a device's
/// DMA or MSIs somewhere the table does not say, if guessed past.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IortError {
    /// A field lies past the table's or its node's end.
    Short,
    /// A node too short for its type, or one overlapping the header.
    BadNode,
    /// An ID mapping whose range wraps, whose output names no node, or that
    /// is a single mapping on a root complex.
    BadMapping,
    /// More root complexes or map entries than the records hold.
    TooMany,
    /// One root complex's IDs sent to two SMMUs or two ITS groups: the
    /// broker binds a device through one of each.
    Ambiguous,
    /// An ITS group whose ITS the interrupt controller tables do not name.
    UnknownIts,
}
