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

use crate::security::hardening::speculation::kernel_entry;
use crate::syscall::contract::{dispatch as contract_dispatch, SyscallArgs};
use crate::syscall::numbers::SyscallNumber;
use crate::syscall::types::errnos;

#[no_mangle]
pub(super) extern "C" fn syscall_handler(
    number: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
) -> u64 {
    // A capsule reaching this point last controlled the branch predictors and
    // the return stack. Refilling the RSB and re-asserting IBRS before any
    // kernel branch runs is the whole point of the entry side, and it was the
    // side with no caller: `kernel_exit` was wired on the return path, so
    // mitigations were being applied leaving the kernel but not entering it.
    kernel_entry();

    let Some(sc) = SyscallNumber::from_u64(number) else {
        /*
         * A number this kernel does not know. NONOS numbers are four
         * character tags, so nothing legitimate lands here; a foreign
         * binary's own numbering does. When the caller is a guest, its
         * supervisor answers and the kernel stays ignorant of what was
         * asked. Everyone else still gets ENOSYS.
         */
        let args = [arg1, arg2, arg3, arg4, arg5, arg6];
        return match crate::process::foreign::redirect(number, args, 0) {
            Some(value) => value,
            None => (-(errnos::ENOSYS as i64)) as u64,
        };
    };
    crate::process::accounting::bump(
        crate::process::current_pid().unwrap_or(0),
        crate::process::accounting::Kind::Syscall,
    );
    crate::process::accounting::bump_total(crate::process::accounting::Total::Syscalls);
    let result = contract_dispatch(sc, SyscallArgs::new([arg1, arg2, arg3, arg4, arg5, arg6]));
    result.value as u64
}
