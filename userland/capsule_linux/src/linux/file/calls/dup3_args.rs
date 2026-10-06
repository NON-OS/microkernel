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

/*
 * dup3's arguments, checked in the order Linux's ksys_dup3 checks them
 * (fs/file.c). Pure, so the host proofs hold it.
 */

use crate::linux::abi::errno;

/* O_CLOEXEC, the one flag dup3 takes (include/uapi/asm-generic/fcntl.h). */
const O_CLOEXEC: u64 = 0o2000000;

/*
 * Whether the copy is close-on-exec. The flags are an int and the numbers
 * unsigned ints, so only their low halves are read. A flag other than
 * O_CLOEXEC is EINVAL, and so is a copy onto itself: dup2 answers that with
 * the number, dup3 refuses it, so a caller cannot take it for a copy that
 * changed the flag.
 */
pub fn dup3_args(from: u64, to: u64, flags: u64) -> Result<bool, i64> {
    let flags = u64::from(flags as u32);
    if flags & !O_CLOEXEC != 0 || from as u32 == to as u32 {
        return Err(errno::EINVAL);
    }
    Ok(flags & O_CLOEXEC != 0)
}
