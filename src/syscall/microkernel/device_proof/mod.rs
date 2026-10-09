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

//! The device proof's kernel calls: this machine's device secret, its two boot
//! slots, and the TPM's half of registering it. For nonos.prove alone.

mod boot_slots;
mod device_secret;
mod enroll;
mod gate;

pub(in crate::syscall::microkernel) use boot_slots::sys_boot_slots;
pub(in crate::syscall::microkernel) use device_secret::sys_device_secret;
pub(in crate::syscall::microkernel) use enroll::sys_enroll;
