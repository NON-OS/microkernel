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
 * `MkDataFeedBegin(name, len, sha256, bytes, probe)`: begin feeding the file
 * `name`, `bytes` long with the SHA-256 the caller names, into the data
 * volume, or take up the stream for it where it stopped. Returns the byte to
 * feed from; EALREADY when the file was imported and verified before. With
 * `probe` 1 it only says where a stream would start and holds nothing.
 * Needs StreamImport, which the model fetcher alone holds.
 */

use super::super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};
use super::feed_errno::{feed_errno, ERRNO_ALREADY};
use super::name::copy_name;
use crate::fs::blockfs_volume::{stream_begin, Begun};

pub fn sys_data_feed_begin(name_ptr: u64, name_len: u64, sha: u64, bytes: u64, probe: u64) -> i64 {
    let caps = crate::syscall::caps::current_caps_or_default();
    let Some(pid) = crate::process::current_pid().filter(|_| caps.can_stream_import()) else {
        return ERRNO_PERM;
    };
    let (name, len) = match copy_name(name_ptr, name_len) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if bytes == 0 || probe > 1 {
        return ERRNO_INVAL;
    }
    let mut want = [0u8; 32];
    if sha == 0 || crate::usercopy::copy_from_user(sha, &mut want).is_err() {
        return ERRNO_FAULT;
    }
    match stream_begin(pid, &name[..len], &want, bytes, probe == 1) {
        Ok(Begun::From(at)) => at as i64,
        Ok(Begun::Done(_)) => ERRNO_ALREADY,
        Err(e) => feed_errno(e),
    }
}
