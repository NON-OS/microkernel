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

//! One backdoor call through the kernel, which allows these calls alone.

use nonos_libc::mk_hv_backdoor;

/// EAX, EBX, ECX and EDX, or the kernel's refusal.
pub fn call(cmd: u32, arg: u32) -> Result<[u32; 4], i64> {
    let mut regs = [0u32; 4];
    match mk_hv_backdoor(cmd, arg, &mut regs) {
        0 => Ok(regs),
        e => Err(e),
    }
}
