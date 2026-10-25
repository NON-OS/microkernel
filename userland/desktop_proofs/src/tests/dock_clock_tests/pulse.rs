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

//! A launch pulse ends, whatever the clock reads.

use super::rig::calc;
use crate::taskbar::{expire_taskbar_pulses, mark_taskbar_launch, new_taskbar_state, Uptime};

#[test]
fn a_launch_pulse_ends_on_time() {
    let mut t = new_taskbar_state();
    mark_taskbar_launch(&mut t, calc(), Uptime(10_000));
    assert!(!expire_taskbar_pulses(&mut t, Uptime(10_899)));
    assert!(expire_taskbar_pulses(&mut t, Uptime(10_900)));
    assert_eq!(t.pulse_until_ms[calc()], 0);
}

/// Set at one reading, then the clock read two hours less: the pulse ends at
/// once instead of after the two hours.
#[test]
fn a_launch_pulse_ends_when_the_clock_reads_behind_it() {
    let mut t = new_taskbar_state();
    let set = 9 * 3_600_000;
    mark_taskbar_launch(&mut t, calc(), Uptime(set));
    assert!(expire_taskbar_pulses(&mut t, Uptime(set - 2 * 3_600_000)));
    assert_eq!(t.pulse_until_ms[calc()], 0);
}

/// A pulse set at a reading at or below zero (the wall clock's error before
/// the RTC is read was -61) is still a pulse, and still ends.
#[test]
fn a_pulse_set_at_an_early_reading_ends() {
    let mut t = new_taskbar_state();
    mark_taskbar_launch(&mut t, calc(), Uptime(-61));
    assert_ne!(t.pulse_until_ms[calc()], 0);
    assert!(expire_taskbar_pulses(&mut t, Uptime(5_000)));
}
