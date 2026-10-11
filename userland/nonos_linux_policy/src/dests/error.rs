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

//! Why a destination list was refused.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestsError {
    /// The bytes end inside the count or an entry.
    Short,
    /// No entries, or more than MAX_ENTRIES.
    Count,
    /// An entry kind no build has given a meaning to.
    Kind,
    /// A first port of 0, or a first port past the last.
    Ports,
    /// An address of the wrong length, a prefix longer than the address, or
    /// an IPv4 address written as IPv6.
    Address,
    /// An address with a bit set past its prefix.
    HostBits,
    /// A name that is not lowercase letters, digits and hyphens in dotted
    /// labels, or that reads as a number, or a name with a prefix.
    Name,
    /// An entry not after the one before it: out of order, or repeated.
    Order,
    /// Bytes after the last entry.
    Trailing,
}
