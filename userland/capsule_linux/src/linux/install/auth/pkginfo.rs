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

//! The `datahash` a package's control member records for its data.

use super::super::tar::entries;

/// The SHA-256 `.PKGINFO` in `control_tar` says the data member hashes to.
pub fn datahash(control_tar: &[u8]) -> Option<[u8; 32]> {
    let found = entries(control_tar).into_iter().find(|e| e.name == b".PKGINFO")?;
    let text = core::str::from_utf8(&found.body).ok()?;
    let hex = text.lines().find_map(|line| line.strip_prefix("datahash = "))?;
    let hex = hex.trim().as_bytes();
    if hex.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (slot, pair) in out.iter_mut().zip(hex.chunks(2)) {
        *slot = (nibble(pair[0])? << 4) | nibble(pair[1])?;
    }
    Some(out)
}

fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    }
}
