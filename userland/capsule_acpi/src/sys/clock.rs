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

//! Time for the interpreter's Sleep, Stall and Timer.

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

pub fn now_ms() -> u64 {
    mk_uptime_ms().max(0) as u64
}

pub fn sleep_ms(ms: u64) {
    let until = now_ms().saturating_add(ms);
    while now_ms() < until {
        let _ = mk_idle_ms(until - now_ms());
    }
}

/// Stall (ACPI 6.5, 19.6.129) is a busy wait of at least `us`. The clock
/// here counts milliseconds, so a stall waits for the next millisecond
/// boundary past it: longer than asked, which a stall may always be, and
/// never shorter.
pub fn stall_us(us: u64) {
    let until = now_ms().saturating_add(us.div_ceil(1000)).saturating_add(1);
    while now_ms() < until {
        core::hint::spin_loop();
    }
}
