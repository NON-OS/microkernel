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

use nonos_inflate::{members_checked, Refused};

use super::super::unpacked::MAX_INFLATED;
use super::digest::{sha1, sha256};
use super::pkginfo::datahash;
use super::verified::Verified;

/// The package's files, if its control member hashes to `checksum` from a
/// signed index and its data hashes to the `datahash` that control records.
/// Either alone leaves something unvouched: the index names only the
/// control member, and only the control member names the data. When memory
/// runs out first, how much was asked, so the person is told that, not that
/// the package was bad.
pub fn verified(apk: &[u8], checksum: &[u8; 20]) -> Result<Verified, Refused> {
    let mut parts = members_checked(apk, MAX_INFLATED)?;
    let [_signature, control, first, ..] = parts.as_slice() else {
        return Err(Refused::Bad);
    };
    let control_ok = apk.get(control.start..control.end).is_some_and(|c| sha1(c) == *checksum);
    let data_ok = match (apk.get(first.start..), datahash(&control.body)) {
        (Some(data), Some(want)) => sha256(data) == want,
        _ => false,
    };
    if !(control_ok && data_ok) {
        return Err(Refused::Bad);
    }
    // The usual package has one data member: its body is moved, not copied,
    // so its files never cost two blocks the size of the package.
    let mut data = parts.split_off(2);
    if data.len() == 1 {
        return Ok(Verified::checked(data.remove(0).body));
    }
    let total = data.iter().map(|p| p.body.len()).sum();
    let mut files: Vec<u8> = Vec::new();
    files.try_reserve_exact(total).map_err(|_| Refused::NoMemory(total))?;
    for part in &data {
        files.extend_from_slice(&part.body);
    }
    Ok(Verified::checked(files))
}
