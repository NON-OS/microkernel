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

use x86_64::registers::control::Cr2;

/*
 * Raw read: `Cr2::read` panics when CR2 is not canonical, and QEMU TCG
 * loads CR2 on a non-canonical access too. The fault is still handled
 * (and a user process killed) instead of panicking the kernel.
 */
pub(super) fn fault_address() -> u64 {
    Cr2::read_raw()
}
