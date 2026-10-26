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

//! Loading one pack for a run: the index read, and held to the format and
//! to the trailer this machine minted over it.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{self, VfsStream};
use nonos_libc::mk_local_verify;
use nonos_linux_pack::{Index, HEADER_LEN};

use super::table::Loaded;

/// The most a trailer is; anything near this is not one.
const MAX_TRAILER: u32 = 4096;

/// One pack's index, refused whole if any of it is not the format's or not
/// this machine's: a pack that does not load serves nothing.
pub fn load(pid: u32, path: &[u8]) -> Result<Loaded, &'static str> {
    let (len, _) = vfs::stat(pid, path)?;
    let len = usize::try_from(len).map_err(|_| "too large")?;
    let mut stream = VfsStream::open(pid, path)?;
    let first = stream.read_window(0, HEADER_LEN as u32)?;
    let head_len = Index::data_start(&first, len).map_err(|_| "not a pack")?;
    let head = stream.read_window(0, head_len as u32)?;
    let trailer = vfs::read_file(pid, &[path, super::paths::TRAILER_AFTER].concat(), MAX_TRAILER)?;
    if !mk_local_verify(&head, 0, &trailer) {
        return Err("its index is not this machine's");
    }
    let head: &'static [u8] = Vec::leak(head);
    let index = Index::parse(head, len).map_err(|_| "its index is not the format's")?;
    Ok(Loaded { index, stream })
}
