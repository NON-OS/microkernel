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

//! A downloaded package, checked against the index record that named it.

use alloc::vec::Vec;

use nonos_inflate::members;

use super::digest::{sha1, sha256};
use super::pkginfo::datahash;
use super::verified::Verified;

/// The package's files, if its control member hashes to `checksum` from a
/// signed index and its data hashes to the `datahash` that control records.
/// Either alone leaves something unvouched: the index names only the
/// control member, and only the control member names the data.
pub fn verified(apk: &[u8], checksum: &[u8; 20]) -> Option<Verified> {
    let parts = members(apk)?;
    let [_signature, control, data @ ..] = parts.as_slice() else {
        return None;
    };
    let first = data.first()?;
    if sha1(apk.get(control.start..control.end)?) != *checksum {
        return None;
    }
    if sha256(apk.get(first.start..)?) != datahash(&control.body)? {
        return None;
    }
    let mut files: Vec<u8> = Vec::new();
    for part in data {
        files.extend_from_slice(&part.body);
    }
    Some(Verified::checked(files))
}
