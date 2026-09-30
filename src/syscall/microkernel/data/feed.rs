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
 * `MkDataFeed(buf, len, end)`: seal the next `len` bytes, at most 1 MiB, of
 * the caller's stream into the data volume, hashing them as they are sealed.
 * Then, with `end` 1, finish: the file is linked under its name only when
 * every byte came and the SHA-256 is the one named at the start (EBADMSG
 * otherwise, and the stream is discarded); with `end` 2, put the stream
 * down with its mark saved. Returns how far the stream has come, or the
 * file's size once finished. Needs StreamImport.
 */

use alloc::vec;

use super::super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};
use super::feed_errno::feed_errno;
use crate::fs::blockfs_volume::{stream_finish, stream_write};

const MAX_FEED: u64 = 1 << 20;
/* Bytes copied in and sealed between two answers to TLB shootdowns. */
const PIECE: u64 = 64 << 10;

pub fn sys_data_feed(buf: u64, len: u64, end: u64) -> i64 {
    let caps = crate::syscall::caps::current_caps_or_default();
    let Some(pid) = crate::process::current_pid().filter(|_| caps.can_stream_import()) else {
        return ERRNO_PERM;
    };
    if len > MAX_FEED || end > 2 || (len == 0 && end == 0) || (len > 0 && buf == 0) {
        return ERRNO_INVAL;
    }
    let mut piece = vec![0u8; len.min(PIECE) as usize];
    let (mut done, mut at) = (0u64, 0u64);
    while done < len {
        /*
         * Each piece is sealed with interrupts masked; shootdowns are
         * answered between pieces, as the disk import does between chunks.
         */
        crate::smp::serve_shootdowns();
        let take = &mut piece[..(len - done).min(PIECE) as usize];
        if crate::usercopy::copy_from_user(buf + done, take).is_err() {
            return ERRNO_FAULT;
        }
        at = match stream_write(pid, take) {
            Ok(at) => at,
            Err(e) => return feed_errno(e),
        };
        done += take.len() as u64;
    }
    match end {
        0 => at as i64,
        keep => stream_finish(pid, keep == 2).map_or_else(feed_errno, |n| n as i64),
    }
}
