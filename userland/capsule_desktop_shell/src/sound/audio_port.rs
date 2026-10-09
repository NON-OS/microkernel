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

//! Where the audio service answers, looked up once and kept.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_service_lookup;

const SERVICE: &[u8] = b"audio.server";

static PORT: AtomicU32 = AtomicU32::new(0);

/// The service's port, or None while it is not registered.
pub(super) fn port() -> Option<u32> {
    let port = PORT.load(Ordering::Relaxed);
    if port != 0 {
        return Some(port);
    }
    let mut found = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut found, &mut pid);
    if rc < 0 || found == 0 {
        return None;
    }
    PORT.store(found, Ordering::Relaxed);
    Some(found)
}

/// A call failed: the service may have restarted on a new port, so the next
/// call looks it up again.
pub(super) fn forget() {
    PORT.store(0, Ordering::Relaxed);
}
