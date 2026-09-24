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

//! One line on the kernel log naming the request that failed, so a failed
//! install can be read from the serial log and not only from its screen.
//! Every failure path goes through here: a refusal by the caller's own
//! checks, a transport error from the kernel, and a status from the driver.

use crate::error::BlkError;

pub fn refused(op: &str, lba: u64, sectors: usize, e: &BlkError) {
    let line =
        alloc::format!("[BLK] {op} failed lba={lba} sectors={sectors} code={} {e:?}\n", e.code());
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
