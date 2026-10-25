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

//! One tracked path against the blob git records for it: the bytes on disk
//! hashed as git hashes a blob, the kind (file or link) and executable bit
//! held to the recorded mode.

use super::blobs::Entry;
use crate::release::read::open_regular;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Read;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Whether the file on disk is the blob git records, as git records it.
pub fn same(tree: &Path, e: &Entry) -> Result<bool, String> {
    let at = tree.join(&e.path);
    let content = match e.mode.as_str() {
        "120000" => match fs::read_link(&at) {
            Ok(target) => target.as_os_str().as_bytes().to_vec(),
            Err(_) => return Ok(false),
        },
        "100644" | "100755" => match open_regular(&at) {
            Ok((f, len)) => match read_all(f, len, e.mode == "100755")? {
                Some(bytes) => bytes,
                None => return Ok(false),
            },
            Err(_) => return Ok(false),
        },
        other => return Err(format!("{} is a git entry of mode {other} (a submodule?)", e.path)),
    };
    let mut h = Sha1::new();
    h.update(format!("blob {}\0", content.len()));
    h.update(&content);
    Ok(hex::encode(h.finalize()) == e.id)
}

// The file's bytes, or none when its executable bit is not the one recorded
// or it changed length while it was read.
fn read_all(f: fs::File, len: u64, exec: bool) -> Result<Option<Vec<u8>>, String> {
    let md = f.metadata().map_err(|e| e.to_string())?;
    if (md.permissions().mode() & 0o100 != 0) != exec {
        return Ok(None);
    }
    let mut bytes = Vec::with_capacity(len as usize);
    f.take(len + 1).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    Ok((bytes.len() as u64 == len).then_some(bytes))
}
