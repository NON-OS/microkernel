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

//! An enrolment's boot applications: the boot manager and anything else
//! this machine's firmware starts besides the release's loader, pinned
//! when the machine was enrolled and signed with the rest. The section
//! carries its own version, so a client that does not know a later
//! meaning for it refuses the certificate rather than misreading it.

use super::Enrol;
use crate::{Reader, WireError};

/// The one meaning this client knows: SHA-256 Authenticode digests.
pub const BOOT_APPS_VERSION: u8 = 1;
/// Most boot applications one machine is enrolled with.
pub const MAX_BOOT_APPS: usize = 8;
pub const BOOT_APP_LEN: usize = 32;

/// The section: its version, a count, then that many digests, none zero
/// and none listed twice.
pub(crate) fn read<'a>(r: &mut Reader<'a>) -> Result<&'a [u8], WireError> {
    if r.u8()? != BOOT_APPS_VERSION {
        return Err(WireError::BodyVersion);
    }
    let n = usize::from(r.u8()?);
    if n > MAX_BOOT_APPS {
        return Err(WireError::Value);
    }
    let apps = r.take(n * BOOT_APP_LEN)?;
    let mut digests = apps.chunks_exact(BOOT_APP_LEN);
    for i in 0..n {
        let a = digests.next().ok_or(WireError::Short)?;
        let again = apps.chunks_exact(BOOT_APP_LEN).take(i).any(|b| b == a);
        if again || a.iter().all(|b| *b == 0) {
            return Err(WireError::Value);
        }
    }
    Ok(apps)
}

impl<'a> Enrol<'a> {
    /// The machine's boot applications, in the order they were enrolled.
    pub fn boot_applications(&self) -> impl Iterator<Item = &'a [u8; BOOT_APP_LEN]> {
        self.boot_apps.chunks_exact(BOOT_APP_LEN).filter_map(|c| c.try_into().ok())
    }

    /// Whether PCR 4 may hold this application on this machine.
    pub fn boot_application(&self, digest: &[u8; BOOT_APP_LEN]) -> bool {
        self.boot_applications().any(|a| a == digest)
    }
}
