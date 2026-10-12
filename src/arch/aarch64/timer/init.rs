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

use super::control::enable_timer_access;
use super::generic::freq_register;
use super::state::{set_frequency, tree_frequency};

/// Latch the counter's rate. CNTFRQ_EL0 when firmware programmed it, else
/// the timer node's `clock-frequency`, which the binding (arm,arch_timer)
/// keeps for exactly that firmware. With neither, no rate is known and no
/// clock is started: every duration would be a guess.
pub fn init_timer() -> Result<(), &'static str> {
    let hz = match freq_register() {
        0 => tree_frequency(),
        hz => hz,
    };
    if hz == 0 {
        return Err("CNTFRQ_EL0 reads 0 and the device tree's timer gives no clock-frequency");
    }
    if freq_register() == 0 {
        crate::sys::serial::Line::new()
            .str(b"[TIMER] CNTFRQ_EL0 reads 0; counter rate from the device tree: ")
            .dec(hz)
            .str(b" Hz")
            .end();
    }
    set_frequency(hz);
    enable_timer_access();
    Ok(())
}

pub fn init_timer_cpu() {
    enable_timer_access();
}
