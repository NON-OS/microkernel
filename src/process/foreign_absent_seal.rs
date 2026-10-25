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

//! The family seal where no guest can exist (see foreign_absent.rs): no
//! family is ever opened here, so none is sealed, measured or read, and
//! the two calls answer ERRNO_NOSYS as every foreign call does.

use crate::syscall::microkernel::errnos::ERRNO_NOSYS;

pub(crate) fn sys_seal_note(_slot: u64, _ptr: u64) -> i64 {
    ERRNO_NOSYS
}

pub(crate) fn sys_seal_read(_app_id_ptr: u64, _out_ptr: u64, _out_len: u64, _pid: u64) -> i64 {
    ERRNO_NOSYS
}

/// No supervisor here has a live family.
pub(crate) fn supervises_live(_sup: u32) -> bool {
    false
}

/// A measurement no family takes.
pub(crate) fn measured(_measurement: &[u8; 32]) {}

/// A guest naming a number the kernel knows gets EPERM before the cap table,
/// the rule x86_64 keeps in seal/absorb.rs. No guest exists here today, so it
/// is never true; the handlers ask it anyway, so the rule is already in place
/// the day one can.
pub(crate) fn native(_nr: u64) -> bool {
    crate::process::current_pid().is_some_and(super::is_foreign)
}
