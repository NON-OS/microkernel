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

use crate::sys::apic::{setup_timer, stop_timer};

// 10 ms slice. Lines up with scheduler::preemption::tick's per-tick
// decrement of CURRENT_TIME_SLICE.
const TICK_HZ: u32 = 100;

// BSP path. Programs this CPU's LAPIC timer. The tick itself is served by
// the loaded IDT's `timer_trampoline` gate, shared by every CPU, so APs
// only program their own LAPIC via `install_on_ap`.
pub fn install_on_bsp() -> Result<(), &'static str> {
    setup_timer(TICK_HZ);
    Ok(())
}

pub fn install_on_ap() {
    setup_timer(TICK_HZ);
}

pub fn disable() {
    stop_timer();
}
