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

//! The two files nonos.prove reads, from the data volume by name with
//! FileSystem: the verifier's request and the registry transcript, brought in
//! by the person (`nonos.prove.fetch`, meant to fetch them, does not exist
//! yet). Nothing is fetched here.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_data_read, mk_data_stat};

pub const REQUEST: &[u8] = b"/prove.request";
pub const TRANSCRIPT: &[u8] = b"/registry.transcript";

/// The most `MkDataRead` reads in one call.
const CHUNK: usize = 4 << 20;
const EIO: i64 = -5;

/// `name` whole, or its first `cap + 1` bytes when it is longer, so the
/// reader refuses it by size; a negative errno when it cannot be read.
pub fn read(name: &[u8], cap: usize) -> Result<Vec<u8>, i64> {
    let size = mk_data_stat(name);
    if size < 0 {
        return Err(size);
    }
    let want = usize::try_from(size).map_or(cap + 1, |n| n.min(cap + 1));
    let mut out = vec![0u8; want];
    let mut at = 0;
    while at < want {
        let end = (at + CHUNK).min(want);
        let got = mk_data_read(name, at as u64, out.get_mut(at..end).ok_or(EIO)?);
        if got < 0 {
            return Err(got);
        }
        if got == 0 {
            return Err(EIO);
        }
        at += got as usize;
    }
    Ok(out)
}
