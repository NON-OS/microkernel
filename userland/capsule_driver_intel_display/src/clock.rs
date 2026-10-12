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

//! The engine's clock: the kernel's monotonic uptime, which counts in
//! milliseconds, read as microseconds. A deadline may run up to a
//! millisecond long; every wait the engine makes is longer than that.

use nonos_intel_display::engine::access::Clock;
use nonos_libc::mk_uptime_ms;

pub struct Uptime;

impl Clock for Uptime {
    fn now_us(&self) -> Option<u64> {
        let ms = mk_uptime_ms();
        (ms >= 0).then(|| (ms as u64).saturating_mul(1000))
    }
}
