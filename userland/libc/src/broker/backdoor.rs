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

use crate::syscall::{call_raw, N_MK_HV_BACKDOOR};

/// One VMware backdoor call: the absolute pointer's commands and the
/// version, on a host that answers them. `out` gets EAX, EBX, ECX and EDX.
#[no_mangle]
pub extern "C" fn mk_hv_backdoor(cmd: u32, arg: u32, out: *mut [u32; 4]) -> i64 {
    call_raw(N_MK_HV_BACKDOOR, [cmd as u64, arg as u64, out as u64, 0, 0, 0])
}
