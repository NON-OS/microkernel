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

//! Where the market capsule is, and a request id to reach it with.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_service_lookup;

/// The service the market capsule announces itself under.
const SERVICE: &[u8] = b"market.index";

/// Request ids only have to differ from this process's other in-flight
/// calls, so a counter is enough and it never needs to survive a restart.
static NEXT_ID: AtomicU32 = AtomicU32::new(1);

pub fn next_id() -> u32 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// The market's port, or zero when it has not announced one.
pub fn port() -> u32 {
    let mut pid: u32 = 0;
    let mut port: u32 = 0;
    let rc = mk_service_lookup(
        SERVICE.as_ptr(),
        SERVICE.len(),
        &mut port as *mut u32,
        &mut pid as *mut u32,
    );
    if rc < 0 || pid == 0 {
        return 0;
    }
    port
}
