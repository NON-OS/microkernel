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
pub(super) use super::microkernel_handle::handle;
use crate::syscall::numbers::SyscallNumber;

mod ops1;
mod ops2;
mod ops3;
mod ops4;

/// The Mk* calls the microkernel dispatch handles, kept in four parts.
pub(super) fn matches(nr: SyscallNumber) -> bool {
    ops1::matches(nr) || ops2::matches(nr) || ops3::matches(nr) || ops4::matches(nr)
}
