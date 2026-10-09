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

//! Says on the log when the bound interface has gone NO_LEASE_SAY_MS without
//! a DHCP lease.

use nonos_libc::{mk_debug, mk_uptime_ms};
use spin::Mutex;

use crate::iface::lease_wait::LeaseWait;
use crate::{setup, state};

static WAIT: Mutex<LeaseWait> = Mutex::new(LeaseWait::new());

pub fn check() {
    let leased = state::lease().is_some_and(|l| l.bound);
    if WAIT.lock().observe(mk_uptime_ms(), setup::bound_port(), leased) {
        let line = b"[NET-CORE] dhcp: no lease 15 s after the link came up\n";
        mk_debug(line.as_ptr(), line.len());
    }
}
