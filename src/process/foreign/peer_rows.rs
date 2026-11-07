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

//! Rows of a guest's pixels, or any strided bytes, read in one call: what a
//! window's changed box costs a supervisor, one call however many rows it
//! has, and none of the bytes between them.

use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::translate_in_asid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};

use super::peer_chunk::{chunk_copy, validate};
use super::peer_guard::{in_user_half, supervised_asid, MAX_SPAN, PAGE};
use super::peer_rows_shape::Rows;

/// `MkPeerReadRows`: `count` rows of `row` bytes from guest `pid`, row `i`
/// at `guest_addr + i * stride`, each to `buf + i * stride`; the bytes
/// between the rows are not touched on either side. Returns the bytes moved.
/// On a fault the rows before it are moved and the call fails.
pub fn sys_peer_read_rows(
    pid: u64,
    guest_addr: u64,
    buf: u64,
    row: u64,
    count: u64,
    stride: u64,
) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let (asid, _held) = match supervised_asid(caller, pid) {
        Ok(pair) => pair,
        Err(e) => return e,
    };
    let Some(rows) = Rows::new(row, count, stride, MAX_SPAN) else {
        return ERRNO_INVAL;
    };
    let Some(span) = rows.span() else {
        return ERRNO_INVAL;
    };
    if !in_user_half(guest_addr, span) || buf.checked_add(span).is_none() {
        return ERRNO_INVAL;
    }
    if !rows_writable(&rows, buf) {
        return ERRNO_FAULT;
    }
    for i in 0..rows.count {
        let Some(at) = rows.at(i) else {
            return ERRNO_INVAL;
        };
        if read_row(asid, guest_addr + at, buf + at, rows.row).is_err() {
            return ERRNO_FAULT;
        }
    }
    rows.bytes() as i64
}

/// Whether every row of the caller's `buf` may be written, checked before
/// any guest page is read, as a whole copy checks its buffer.
fn rows_writable(rows: &Rows, buf: u64) -> bool {
    (0..rows.count)
        .all(|i| rows.at(i).is_some_and(|at| validate(buf + at, rows.row, false).is_ok()))
}

/// One row of `len` bytes from guest address `va` to the caller's `buf`, a
/// page at a time. The caller's side was validated before the first row and
/// is checked again by each page's copy.
fn read_row(asid: u32, va: u64, buf: u64, len: u64) -> Result<(), ()> {
    let mut done = 0u64;
    while done < len {
        let at = va + done;
        let page = at & !(PAGE - 1);
        let offset = at - page;
        let chunk = core::cmp::min(PAGE - offset, len - done);
        let phys = translate_in_asid(asid, VirtAddr::new(page)).ok_or(())?;
        chunk_copy(phys, offset, buf + done, chunk, false)?;
        done += chunk;
    }
    Ok(())
}
