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

use super::info::BootInfo;
use crate::arch::aarch64::timer;

/// The counter's rate, the clock every duration is measured on, and this
/// CPU's tick. Refuses the boot when the rate is unknown: a scheduler on a
/// guessed clock runs every timeout at the wrong length without saying so.
pub(super) fn start_clock(boot_info: &BootInfo) {
    if let Err(reason) = timer::init_timer() {
        super::refuse(b"timer", reason.as_bytes());
    }
    // Once the rate is latched, as x86 does in its entry. Unanchored, time
    // scales by the 2.5 GHz guessed for a TSC: a Pi's 54 MHz runs 46x slow.
    crate::time::anchor();
    timer::configure_preemption_intid(boot_info.timer_phys_intid);
    if let Err(reason) = timer::install_on_cpu() {
        super::refuse(b"timer tick", reason.as_bytes());
    }
}
