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

//! The two things the controller code needs from time: what the uptime clock
//! reads, and a sleep. Kept apart so the controller source the host proofs
//! include (`userland/hda_proofs`) runs against the host's clock instead.

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

/// Milliseconds since boot, or zero when the clock cannot be read. A wait
/// built on this also counts its polls (`controller::wait`), so a clock that
/// never moves still ends it.
pub fn now_ms() -> u64 {
    let t = mk_uptime_ms();
    if t < 0 {
        0
    } else {
        t as u64
    }
}

/// Sleep at least `ms` milliseconds without holding a core.
pub fn pause_ms(ms: u64) {
    let _ = mk_idle_ms(ms);
}
