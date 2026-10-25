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

//! Whether an install from a mirror can start now.

use alloc::format;

use super::super::route_read::offline;
use super::super::Why;
use super::anyone::down;

/// Whether an install from `at` can start now: in a development build
/// (`dev-mirror`) a mirror on the local network at once; any other once Anyone, which installs download over,
/// is up, waited for up to three minutes (`anyone::down`). When
/// it does not come up, that is said before anything is fetched, as
/// NoNetwork: an outage, which a retry once it runs gets past, not a mirror
/// that refused.
pub fn reachable(at: &str) -> Result<(), Why> {
    match offline(local(at), down) {
        None => Ok(()),
        Some(why) => {
            let line = format!("[LINUX] mirror {at}: not reachable now: {why}\n");
            let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
            Err(Why::NoNetwork)
        }
    }
}

#[cfg(feature = "dev-mirror")]
fn local(at: &str) -> bool {
    crate::linux::net::route::is_local(at)
}

/* No release build dials a mirror but through the route layer. */
#[cfg(not(feature = "dev-mirror"))]
fn local(_: &str) -> bool {
    false
}
