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

//! Binding the stack to the best up link.

use core::sync::atomic::Ordering;

use super::bound::BOUND_PORT;
use super::discover::discover_nic;
use super::log::{bind_log, bind_up_log};
use crate::device;
use crate::iface::build;
use crate::state;

/// Bind, or rebind, the stack to the best up link. The first call after boot with
/// a link up does the initial bind (nothing is bound, so any up link differs from
/// the zero sentinel); later calls switch interfaces when the WiFi link associates
/// or the bound link drops. A no-op once bound to the best link, so it is cheap to
/// call on a timer from the server loop.
pub fn reevaluate() {
    let Some((best, name)) = discover_nic() else {
        return;
    };
    if best == BOUND_PORT.load(Ordering::Acquire) {
        return;
    }
    // Both remaining exits used to be silent, and a stack that failed here
    // looked identical to one that found no link at all: no bind, no lease,
    // and every consumer reporting its own timeout.
    let Some(mac) = device::mac(best) else {
        bind_log(b"[NET-CORE] bind: mac query failed");
        return;
    };
    let Some(net_state) = build::build(mac, best) else {
        bind_log(b"[NET-CORE] bind: stack build failed");
        return;
    };
    state::store(net_state);
    BOUND_PORT.store(best, Ordering::Release);
    bind_up_log(name);
}
