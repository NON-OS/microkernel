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

extern crate alloc;
use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::{read_file, stat};
use nonos_libc::mk_getpid;

use crate::track_limit::{refuse_size, READ_LIMIT};

/// Read a whole track, refusing one past the limit: by its size before any of
/// it is read when the store can say it, else by a read longer than the limit.
pub fn load(path: &[u8]) -> Result<Vec<u8>, &'static str> {
    let pid = mk_getpid();
    if let Some(why) = stat(pid, path).ok().and_then(|(size, _)| refuse_size(size)) {
        return Err(why);
    }
    let bytes = read_file(pid, path, READ_LIMIT)?;
    match refuse_size(bytes.len() as u64) {
        Some(why) => Err(why),
        None => Ok(bytes),
    }
}
