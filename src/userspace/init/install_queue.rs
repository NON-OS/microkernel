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


//! Package installs asked for by a capsule, performed by init once the
//! market says the listing is ready and names the bytes to expect.

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

use crate::security::market_capsule::client::{queued_get_release, queued_install_ready};
use crate::sys::serial::{print, println};

/// Deeper than a person clicks, shallower than a caller in a loop can grow.
const DEPTH: usize = 8;

/// A listing and the release asked for, which is empty for the default.
static PENDING: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());

/// Record a request. False when full, which the caller reports as busy.
pub(crate) fn request(listing: String, release: String) -> bool {
    let mut q = PENDING.lock();
    if q.len() >= DEPTH || q.iter().any(|(l, _)| *l == listing) {
        return false;
    }
    q.push((listing, release));
    drop(q);
    super::instance_spawn::raise_drain();
    true
}

/// Whether an install is waiting; a contended lock is a push in flight.
pub(crate) fn has_pending() -> bool {
    PENDING.try_lock().map_or(true, |q| !q.is_empty())
}

/// Perform every queued install the market still vouches for.
pub(crate) fn service() {
    let taken = core::mem::take(&mut *PENDING.lock());
    for (listing, release) in taken {
        let Some(name) = listing.strip_prefix("linux.") else { continue };
        /*
         * The store showed the listing as ready, and that was its word. The
         * market's own verdict is asked for again here, and the release's
         * package hash goes to the installer, which refuses any other bytes.
         */
        let ready = queued_install_ready(&listing, &release).is_ok_and(|r| r.install_ready);
        let pinned = queued_get_release(&listing, &release).ok().map(|r| r.package_hash);
        let (true, Some(hash)) = (ready, pinned) else {
            print(b"[LINUX-INSTALL] not ready, refused ");
            println(listing.as_bytes());
            continue;
        };
        match crate::userspace::capsule_linux::spawn_install(name, &hash) {
            Ok(_) => print(b"[LINUX-INSTALL] started "),
            Err(_) => print(b"[LINUX-INSTALL] refused "),
        }
        println(listing.as_bytes());
    }
}
