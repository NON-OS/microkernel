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

//! Why a buffer was refused.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WireError {
    /// Fewer octets than the field or the stated length needs.
    Short,
    /// A stated length disagrees with the octets present.
    Length,
    /// More octets than the format or a bound can carry.
    TooLong,
    /// A field holds a value the specification reserves or forbids here.
    Reserved,
    /// An ACL continuation fragment with no start fragment open.
    Sequence,
}
