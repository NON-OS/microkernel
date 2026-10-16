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

//! The router's pid, looked up once and again only when a delivery comes
//! from another pid (a router that restarted has a new one).

use core::sync::atomic::{AtomicU32, Ordering};

use crate::server::handlers::launcher_request::lookup_pid;
use crate::state::input_source_rule::from_router;

const ROUTER: &[u8] = b"input_router";
static KNOWN: AtomicU32 = AtomicU32::new(0);

pub fn is_router(sender: u32) -> bool {
    let known = KNOWN.load(Ordering::Relaxed);
    if known != 0 && from_router(sender, Some(known)) {
        return true;
    }
    let now = lookup_pid(ROUTER);
    KNOWN.store(now.unwrap_or(0), Ordering::Relaxed);
    from_router(sender, now)
}
