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

//! What init asks the market before an install (I-C5): the release, its
//! readiness and its box. The store showed it ready; the market is asked
//! again, and the release's package hash goes to the installer, which takes
//! no other. The release is resolved once: readiness and the box are asked
//! of that release by its own id, never of the word "latest" again, and the
//! box is held to the same release's hash (box_of.rs).

use crate::security::market_capsule::client::{queued_get_release, queued_install_ready};

use super::box_of::{box_of, Arg};

/// The pin and the box, or the line that says why the install is refused.
pub(super) fn vouched(
    listing: &str,
    release: &str,
) -> Result<([u8; 32], Option<Arg>), &'static [u8]> {
    let found = queued_get_release(listing, release);
    let release = found.as_ref().map_or(release, |r| r.release_id.as_str());
    let asked = queued_install_ready(listing, release);
    let ready = asked.as_ref().is_ok_and(|r| r.install_ready);
    let pinned = found.as_ref().map(|r| r.package_hash).map_err(|e| *e);
    super::why::say(&asked, &pinned);
    let (true, Ok(hash)) = (ready, pinned) else {
        return Err(b"[LINUX-INSTALL] not ready, refused ");
    };
    match box_of(listing, release, &hash) {
        Ok(policy) => Ok((hash, policy)),
        Err(()) => Err(b"[LINUX-INSTALL] the market does not vouch for its box, refused "),
    }
}
