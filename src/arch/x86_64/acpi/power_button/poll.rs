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

//! One look at the button from the paced tick, and whether to look at all.

use core::sync::atomic::Ordering;

use super::super::hw::power_button::{clear_value, pressed};
use super::deliver::deliver;
use super::gate::{accept, capsule_serves};
use super::{PENDING, STS_A, STS_B};
use crate::arch::x86_64::port::{inw, outw};
use crate::hardware::broker::acpi::button_owner::Press;

/// The paced tick runs at 100 Hz.
const MS_PER_TICK: u64 = 10;

/// One look at the button, from the boot CPU's paced tick.
pub fn poll(ticks: u64) {
    let sts_a = STS_A.load(Ordering::Acquire);
    if sts_a == 0 || capsule_serves() {
        return;
    }
    let sts_b = STS_B.load(Ordering::Relaxed);
    let now_ms = ticks.saturating_mul(MS_PER_TICK);
    // SAFETY: the ports are the FADT's PM1 status registers, checked to be
    // 16-bit System I/O at init; the bit is write-one-to-clear and only it
    // is written.
    let hit = unsafe {
        let a = pressed(inw(sts_a));
        let b = sts_b != 0 && pressed(inw(sts_b));
        if a {
            outw(sts_a, clear_value());
        }
        if b {
            outw(sts_b, clear_value());
        }
        a || b
    };
    if hit && accept(now_ms) {
        // Once a capsule has served the button, a press while none does is
        // its power-off still; before that, the desktop's Power key.
        match crate::hardware::broker::acpi::button_owner::kernel_press() {
            Press::Desktop => PENDING.store(true, Ordering::Release),
            Press::PowerOff => crate::hardware::broker::acpi::leave_rule::press_in_gap(),
        }
    }
    if PENDING.load(Ordering::Acquire) && deliver() {
        PENDING.store(false, Ordering::Release);
    }
}
