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
//! The kernel's uptime as the engine's clock. Each polling pass sleeps a
//! millisecond, a frame: nothing the controller does finishes sooner, and
//! the core is free meanwhile.

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

use crate::env::Clock;

pub struct Uptime;

impl Clock for Uptime {
    fn now_ms(&self) -> u64 {
        // A refused read as 0 would hold every wait forever; as the end of
        // time it ends each at once, and the request fails instead.
        u64::try_from(mk_uptime_ms()).unwrap_or(u64::MAX)
    }

    fn idle(&self) {
        let _ = mk_idle_ms(1);
    }
}
