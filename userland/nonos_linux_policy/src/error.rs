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

//! Why a policy was refused.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyError {
    /// Not exactly 256 bytes.
    Length,
    /// Not "NLXPOL01".
    Magic,
    /// A version this build does not know.
    Version,
    /// A reserved byte is not zero.
    Reserved,
    /// A mask bit no build has given a meaning to.
    UnknownMask,
    /// A syscall bit past bucket 512.
    SyscallSet,
}
