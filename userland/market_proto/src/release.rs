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

//! The release a listing would install, as `OP_GET_RELEASE` returns it:
//! release id, manifest hash, package hash, package url, publisher
//! signature, arches, minimum kernel ABI, capabilities, validation status,
//! validation note, validator, and when it validated.

use alloc::vec::Vec;

use super::lp::{lp, skip, skip_list};

pub struct Release {
    /// The release id, such as "linux.htop@3.3.0".
    pub version: Vec<u8>,
    /// The operator's validation note, such as the size it hashed.
    pub note: Vec<u8>,
}

impl Release {
    /// The version alone: what follows the last `@` of the release id.
    pub fn short_version(&self) -> &[u8] {
        self.version.rsplit(|b| *b == b'@').next().unwrap_or(&self.version)
    }
}

pub fn parse_release(body: &[u8]) -> Option<Release> {
    let (version, at) = lp(body, 0)?;
    // The url is provenance the installer checks; a client does not show it.
    let at = skip(body, at, 64)?;
    let (_, at) = lp(body, at)?;
    let (_, at) = lp(body, at)?;
    let at = skip(body, skip_list(body, at)?, 4)?;
    let at = skip(body, skip_list(body, at)?, 1)?;
    let (note, _) = lp(body, at)?;
    Some(Release { version: version.to_vec(), note: note.to_vec() })
}
