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
//! Putting a package's files into the store, under the Linux root.
//! A package's pack built and put in the store, with the trailer this
//! machine mints over its index. What fails says why, and leaves nothing of
//! the new pack behind.

use alloc::vec::Vec;

use nonos_libc::mk_local_verify;
use nonos_linux_pack::{build, Index, Source, HEADER_LEN};

use super::super::record::vouch::mint;
use super::super::Why;
use super::admit::{admitted, Unadmitted};
use super::land_said::{landed, names, refused, say};
use crate::linux::file::family::chosen;
use crate::linux::file::pack::{put, remove, Held};

/// A run loads a pack under the trailer minted for a process holding no
/// capabilities (file/store/pack/load.rs).
const RUN_CAPS: u64 = 0;

pub(super) fn land(name: &str, sources: Vec<Source>) -> Result<(), Why> {
    let files = sources.len();
    let bytes = build(sources).map_err(|why| refused(name, why))?;
    let head = Index::data_start(&bytes[..HEADER_LEN], bytes.len()).map_err(|_| Why::PACKAGE)?;
    let serves = |index: &[u8], trailer: &[u8]| mk_local_verify(index, RUN_CAPS, trailer);
    let trailer = admitted(&bytes[..head], mint, serves).map_err(|not| {
        say(name, not.why());
        match not {
            Unadmitted::NoMint => Why::PARTIAL,
            Unadmitted::NotRun => Why::NOT_RUN,
        }
    })?;
    let before = names();
    match put(chosen(), name, &bytes, head, &trailer) {
        Ok(()) => {
            landed(name, files, bytes.len(), before);
            Ok(())
        }
        Err(Held::InUse) => {
            say(name, "a running app holds it open; close the app to update it");
            Err(Why::IN_USE)
        }
        Err(Held::Unwritten) => {
            let _ = remove(chosen(), name);
            say(name, "the store would not take its pack");
            Err(Why::PARTIAL)
        }
        Err(Held::Full) => {
            let _ = remove(chosen(), name);
            let mib = bytes.len().div_ceil(1 << 20);
            say(name, &alloc::format!("the store has no room for its pack of {mib} MiB"));
            Err(Why::PARTIAL)
        }
    }
}
