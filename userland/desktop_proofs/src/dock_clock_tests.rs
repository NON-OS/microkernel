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

//! The dock's timers (state/taskbar: the launch pulse, the brand reveal,
//! the wait for a launched app's window) take uptime only, the `Uptime`
//! type, and a reading behind the time one was set ends it rather than hold
//! it. They were fed the wall clock: before the RTC was read it read an
//! error, and NTP stepped it back by hours on a laptop whose RTC keeps local
//! time, so a pulse or reveal set before the step stayed for its length.

use crate::shell_apps::LAUNCHER_APPS;
use crate::taskbar::{
    expire_taskbar_pulses, expire_taskbar_visibility, mark_taskbar_launch, new_taskbar_state,
    reveal_taskbar, set_full_screen, Uptime,
};

const WINDOW: u32 = 0x4E4F_0001;

fn calc() -> usize {
    LAUNCHER_APPS.iter().position(|a| a.service == b"app.calculator").expect("in the table")
}

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

/// The brand's reveal over a full-screen window lapses after its 1.8 s, and
/// also when the clock reads behind the time it was set.
#[test]
fn a_reveal_lapses_on_time_and_when_the_clock_reads_behind_it() {
    let mut t = new_taskbar_state();
    set_full_screen(&mut t, 300, WINDOW, true);
    let set = 5 * 3_600_000;
    assert!(reveal_taskbar(&mut t, Uptime(set)));
    assert!(t.revealed);
    assert!(!expire_taskbar_visibility(&mut t, Uptime(set + 1_799)));
    assert!(expire_taskbar_visibility(&mut t, Uptime(set + 1_800)));
    assert!(!t.revealed);

    assert!(reveal_taskbar(&mut t, Uptime(set)));
    assert!(expire_taskbar_visibility(&mut t, Uptime(set - 3_600_000)));
    assert!(!t.revealed, "a reveal set later than the clock reads lapses");
}
