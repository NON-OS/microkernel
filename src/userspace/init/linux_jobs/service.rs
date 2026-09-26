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

use crate::security::market_capsule::client::{queued_get_release, queued_install_ready};
use crate::sys::serial::{print, println};
use crate::userspace::capsule_linux::{spawn_install, spawn_run};

use super::queue::{take, Job};

/// Perform every queued job.
pub(crate) fn service() {
    for job in take() {
        match job {
            Job::Install(listing, release) => install(&listing, &release),
            Job::Run(package) => {
                let said: &[u8] = match spawn_run(&package) {
                    Ok(_) => b"[LINUX-RUN] started ",
                    Err(_) => b"[LINUX-RUN] refused ",
                };
                print(said);
                println(package.as_bytes());
            }
        }
    }
}

fn install(listing: &str, release: &str) {
    let Some(name) = listing.strip_prefix("linux.") else { return };
    /*
     * The store showed the listing as ready, and that was its word. The
     * market's own verdict is asked for again here, and the release's
     * package hash goes to the installer, which refuses any other bytes.
     */
    let ready = queued_install_ready(listing, release).is_ok_and(|r| r.install_ready);
    let pinned = queued_get_release(listing, release).ok().map(|r| r.package_hash);
    let said: &[u8] = match (ready, pinned) {
        (true, Some(hash)) => match spawn_install(name, &hash) {
            Ok(_) => b"[LINUX-INSTALL] started ",
            Err(_) => b"[LINUX-INSTALL] refused ",
        },
        _ => b"[LINUX-INSTALL] not ready, refused ",
    };
    print(said);
    println(listing.as_bytes());
}
