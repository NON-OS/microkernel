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

use super::handle::BlockDevice;
use crate::error::BlkError;

/// A request refused before it reached the driver, in 512-byte sectors.
pub fn refused(op: &str, lba: u64, sectors: usize, e: &BlkError) {
    crate::refusal::note(op_of(op), lba, sectors as u32, e.code());
    let line =
        alloc::format!("[BLK] {op} failed lba={lba} sectors={sectors} code={} {e:?}\n", e.code());
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

/// A driver request that failed, in the disk's own blocks as it was sent.
pub fn refused_blocks(op: &str, dev: &BlockDevice, lba: u64, bytes: usize, e: &BlkError) {
    let size = dev.geometry.lba_size();
    let per_block = u64::from(size) / 512;
    crate::refusal::note(op_of(op), lba.saturating_mul(per_block), (bytes / 512) as u32, e.code());
    let line = alloc::format!(
        "[BLK] {op} failed {:?}{} lba={lba} sectors={} block={size} code={} {e:?}\n",
        dev.driver,
        dev.instance,
        bytes / size as usize,
        e.code()
    );
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

fn op_of(op: &str) -> crate::refusal::Op {
    match op {
        "write" => crate::refusal::Op::Write,
        "flush" => crate::refusal::Op::Flush,
        _ => crate::refusal::Op::Read,
    }
}
