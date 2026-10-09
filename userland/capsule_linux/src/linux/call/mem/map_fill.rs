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

//! Filling an executable mapping from the bytes that were proved.

use crate::linux::file::pread64;
use crate::linux::guest::Guest;

use super::map_req::MapReq;

/// Copy the proved file's `[off, off + len)` to `at`. Past the end of the file
/// the fresh frames already read as zero, which is a segment's bss.
pub(super) fn fill_from(guest: &Guest, bytes: &[u8], req: &MapReq, at: u64) -> i64 {
    let Ok(off) = usize::try_from(req.off) else {
        return 0;
    };
    if off >= bytes.len() {
        return 0;
    }
    let len = usize::try_from(req.len).unwrap_or(usize::MAX);
    let end = off.saturating_add(len).min(bytes.len());
    guest.write(at, &bytes[off..end])
}

/// Read `[off, off + len)` of the file into `at`, for a mapping nothing will
/// run. Negative on the first failed read.
pub(super) fn fill_read(guest: &mut Guest, req: &MapReq, at: u64) -> i64 {
    let mut done = 0u64;
    while done < req.len {
        let n = pread64(guest, req.fd, at + done, req.len - done, req.off + done) as i64;
        if n < 0 {
            return n;
        }
        if n == 0 {
            /*
             * Short of the requested span: the rest of the mapping is the
             * zeroes the fresh frames already hold, which is what a segment's
             * bss is.
             */
            break;
        }
        done += n as u64;
    }
    0
}
