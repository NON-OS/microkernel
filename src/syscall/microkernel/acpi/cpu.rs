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

//! `MkAcpiCpu(record, len) -> 0`: the ACPI capsule hands over one
//! processor's _PSS, _PCT, _PPC, _CST and _CPC, as `cpu_layout` lays them
//! out, after its first read and again when a Notify says they changed.
//! EINVAL for a record that is malformed or longer than the layout allows;
//! ENOSPC once every processor slot is taken.

use crate::hardware::broker::acpi::cpu_layout::MAX_RECORD;
use crate::hardware::broker::acpi::cpu_store::{adopt, Adopted};
use crate::hardware::broker::acpi::cpu_wire::decode;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOSPC};
use crate::usercopy::read_user_bytes;

pub fn sys_acpi_cpu(ptr: u64, len: u64) -> i64 {
    let Some(len) = usize::try_from(len).ok().filter(|l| *l <= MAX_RECORD) else {
        return ERRNO_INVAL;
    };
    let Ok(bytes) = read_user_bytes(ptr, len) else { return ERRNO_FAULT };
    let Some(t) = decode(&bytes) else { return ERRNO_INVAL };
    let (uid, ppc) = (t.uid, t.ppc);
    match adopt(t) {
        Adopted::Full => ERRNO_NOSPC,
        Adopted::Changed => {
            crate::log_info!("[ACPI] cpu {}: tables changed, _PPC {:?}", uid, ppc);
            0
        }
        Adopted::New | Adopted::Same => 0,
    }
}
