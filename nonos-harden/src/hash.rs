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

//! SHA-256 of files and bytes, as the seal writes them: lowercase hex.

use crate::release::read::open_regular;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;

/// The most any one sealed file may hold: past it a file is refused unread,
/// so a vast sparse image cannot hold the verdict back.
pub const MAX_FILE: u64 = 64 << 30;

/// The digest of a regular file, read once through one handle.
pub fn sha256_file(path: &Path) -> Result<String, String> {
    let (mut f, len) = open_regular(path)?;
    if len > MAX_FILE {
        return Err(format!("larger than the {MAX_FILE} bytes a sealed file can be"));
    }
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    let mut seen = 0u64;
    loop {
        let n = f.read(&mut buf).map_err(|e| format!("unreadable ({e})"))?;
        seen += n as u64;
        if n == 0 || seen > MAX_FILE {
            break;
        }
        h.update(&buf[..n]);
    }
    match seen <= MAX_FILE {
        true => Ok(hex::encode(h.finalize())),
        false => Err(format!("it grew past {MAX_FILE} bytes while it was read")),
    }
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

/// Sixty-four lowercase hex digits: the only form the seal writes.
pub fn is_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// The first twelve digits, to name a digest in a reason without the noise.
pub fn short(digest: &str) -> &str {
    digest.get(..12).unwrap_or(digest)
}
