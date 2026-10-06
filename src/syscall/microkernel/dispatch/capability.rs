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

use super::args::Args;
use crate::syscall::microkernel::capability::{sys_cap_check, sys_cap_grant, sys_cap_revoke};
use crate::syscall::microkernel::errnos::ERRNO_INVAL;
use crate::syscall::microkernel::narrow::u32_arg;
use crate::syscall::microkernel::numbers::*;

pub(super) fn handle(nr: u64, a: Args) -> Option<i64> {
    let call: fn(u32, u64) -> i64 = match nr {
        SYS_CAP_GRANT => sys_cap_grant,
        SYS_CAP_REVOKE => sys_cap_revoke,
        SYS_CAP_CHECK => sys_cap_check,
        _ => return None,
    };
    // The target pid is refused, not truncated onto another process.
    Some(u32_arg(a.a0).map_or(ERRNO_INVAL, |pid| call(pid, a.a1)))
}
