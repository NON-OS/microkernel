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

extern crate alloc;

use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};

pub fn sys_args(buf: u64, len: usize) -> i64 {
    let Some(pid) = current_pid() else {
        return ERRNO_PERM;
    };
    let Some(pcb) = crate::process::get_process_table().find_by_pid(pid) else {
        return ERRNO_INVAL;
    };
    let mut blob: alloc::vec::Vec<u8> = alloc::vec::Vec::new();
    for (i, arg) in pcb.argv.lock().iter().enumerate() {
        if i > 0 {
            blob.push(0);
        }
        blob.extend_from_slice(arg.as_bytes());
    }
    if buf == 0 || len < blob.len() {
        return blob.len() as i64;
    }
    if crate::usercopy::copy_to_user(buf, &blob).is_err() {
        return ERRNO_FAULT;
    }
    blob.len() as i64
}
