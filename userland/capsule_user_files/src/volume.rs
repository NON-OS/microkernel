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


//! The four object calls, as the broker asks for them.

use nonos_libc::{mk_user_obj_drop, mk_user_obj_put, mk_user_obj_read, mk_user_obj_size};
use nonos_user_files::errno::{Errno, EIO};
use nonos_user_files::Objects;

pub struct Volume;

/// A kernel answer: its value, or the errno it carried.
fn answer(rc: i64) -> Result<u64, Errno> {
    match rc {
        rc if rc >= 0 => Ok(rc as u64),
        rc => Err(i32::try_from(-rc).unwrap_or(EIO)),
    }
}

impl Objects for Volume {
    fn put(&mut self, id: u64, bytes: &[u8]) -> Result<(), Errno> {
        answer(mk_user_obj_put(id, bytes)).map(drop)
    }

    fn read(&mut self, id: u64, offset: u64, buf: &mut [u8]) -> Result<usize, Errno> {
        if buf.is_empty() {
            return Ok(0);
        }
        answer(mk_user_obj_read(id, offset, buf)).map(|n| n as usize)
    }

    fn size(&mut self, id: u64) -> Result<u64, Errno> {
        answer(mk_user_obj_size(id))
    }

    fn remove(&mut self, id: u64) -> Result<(), Errno> {
        answer(mk_user_obj_drop(id)).map(drop)
    }
}
