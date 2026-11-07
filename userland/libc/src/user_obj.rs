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

//! The person's files' objects on the data volume, by number: the
//! user-files broker's alone, each call needing UserFiles. Errnos negative.

use crate::syscall::{
    call_raw, N_MK_USER_OBJ_DROP, N_MK_USER_OBJ_PUT, N_MK_USER_OBJ_READ, N_MK_USER_OBJ_SIZE,
};

/// Writes object `id` as `bytes`, at most 1 MiB, making or replacing it. 0.
pub fn mk_user_obj_put(id: u64, bytes: &[u8]) -> i64 {
    call_raw(N_MK_USER_OBJ_PUT, [id, bytes.as_ptr() as u64, bytes.len() as u64, 0, 0, 0])
}

/// Up to `buf.len()` bytes, at most 1 MiB, of object `id` from `offset`.
/// The bytes read, 0 at its end.
pub fn mk_user_obj_read(id: u64, offset: u64, buf: &mut [u8]) -> i64 {
    call_raw(N_MK_USER_OBJ_READ, [id, offset, buf.as_mut_ptr() as u64, buf.len() as u64, 0, 0])
}

/// Object `id`'s length; ENOENT when there is none.
pub fn mk_user_obj_size(id: u64) -> i64 {
    call_raw(N_MK_USER_OBJ_SIZE, [id, 0, 0, 0, 0, 0])
}

/// Removes object `id`. 0; ENOENT when there was none.
pub fn mk_user_obj_drop(id: u64) -> i64 {
    call_raw(N_MK_USER_OBJ_DROP, [id, 0, 0, 0, 0, 0])
}
