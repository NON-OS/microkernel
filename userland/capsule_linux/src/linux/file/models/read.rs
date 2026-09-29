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

/* Reading a model: a range of it, straight from the volume. */

use alloc::vec;
use alloc::vec::Vec;

use crate::linux::abi::errno;
use nonos_libc::mk_data_read;

use super::name::{owns, volume_name};

/*
 * The bytes at `offset` of the model at `path`, at most `len`; None if the
 * path is not under /models.
 */
pub fn read(path: &[u8], offset: u64, len: usize) -> Option<Result<Vec<u8>, i64>> {
    if !owns(path) {
        return None;
    }
    if path == super::catalog::TIERS {
        let all = super::catalog::text();
        let from = (offset as usize).min(all.len());
        return Some(Ok(all[from..(from + len).min(all.len())].to_vec()));
    }
    let Some(name) = volume_name(path) else {
        return Some(Err(errno::EISDIR));
    };
    if len == 0 {
        return Some(Ok(Vec::new()));
    }
    let mut buf = vec![0u8; len];
    let got = mk_data_read(name, offset, &mut buf);
    Some(if got < 0 {
        Err(-got)
    } else {
        buf.truncate(got as usize);
        Ok(buf)
    })
}
