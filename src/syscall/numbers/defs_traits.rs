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

//! What the derive gave SyscallNumber, written out so defs.rs holds only its
//! variants. A fieldless enum is its discriminant: copied by value, equal
//! when the tags are, and named in Debug by the registry, whose names are
//! the variants' own (kernel_proofs::syscall_tables), as `[CAP-DENY]` lines
//! print them.

use core::fmt;

use super::defs::SyscallNumber;

impl Clone for SyscallNumber {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for SyscallNumber {}

impl PartialEq for SyscallNumber {
    fn eq(&self, other: &Self) -> bool {
        *self as u64 == *other as u64
    }
}

impl Eq for SyscallNumber {}

impl fmt::Debug for SyscallNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match crate::syscall::abi::lookup_name(*self as u64) {
            Some(name) => f.write_str(name),
            None => write!(f, "SyscallNumber({:#x})", *self as u64),
        }
    }
}
