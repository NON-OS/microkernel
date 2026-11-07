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

//! The verifier's blob on disk: `/nonos/lock/record`, written by the shell
//! alone, only on a boot that keeps what is written. It is the record the
//! keyring sealed (or, with no TPM, the record itself); it never holds the
//! passphrase.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_getpid, mk_keeping};
use nonos_lock::blob::BLOB_MAX;

const DIRS: [&[u8]; 2] = [b"/nonos", b"/nonos/lock"];
const PATH: &[u8] = b"/nonos/lock/record";

/// The blob kept on disk, if one is.
pub(super) fn read() -> Option<Vec<u8>> {
    let bytes = vfs::read_file(mk_getpid(), PATH, BLOB_MAX as u32).ok()?;
    (!bytes.is_empty() && bytes.len() <= BLOB_MAX).then_some(bytes)
}

/// Write `blob` and persist it. False when this boot keeps nothing or the
/// store did not take it; the tick tries again.
pub(super) fn write(blob: &[u8]) -> bool {
    if !mk_keeping() {
        return false;
    }
    let pid = mk_getpid();
    for dir in DIRS {
        let _ = vfs::mkdir(pid, dir);
    }
    let _ = vfs::unlink(pid, PATH);
    vfs::write_file(pid, PATH, blob).is_ok() && vfs::persist(pid, PATH).is_ok()
}
