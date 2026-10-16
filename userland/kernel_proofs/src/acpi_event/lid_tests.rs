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

//! The lid told to the shell (F-91), by the pure rule the kernel runs.

use super::lid_state::{decide, Report};

/// F-91: a reading the shell already has is not told again; a change is
/// told, and recorded only once the shell took it, so a busy shell is told
/// at the next reading.
#[test]
fn the_lid_is_told_once_and_again_after_a_busy_shell() {
    let (unknown, closed, open) = (0u8, 1u8, 2u8);
    let never = |_| -> bool { panic!("nothing to tell") };
    assert_eq!(decide(closed, false, never), (Report::Unchanged, closed));
    assert_eq!(decide(open, true, never), (Report::Unchanged, open));
    assert_eq!(decide(unknown, true, |o| o), (Report::Told, open), "the first reading");
    assert_eq!(decide(open, false, |o| !o), (Report::Told, closed));
    let (r, kept) = decide(open, false, |_| false);
    assert_eq!((r, kept), (Report::Busy, open), "busy: nothing recorded");
    assert_eq!(decide(kept, false, |_| true), (Report::Told, closed), "told next time");
}

/// F-96: the output's brightness keys decode with value 1 up and 0 down.
#[test]
fn brightness_keys_decode() {
    use super::platform_event::{decode, Event, BRIGHTNESS};
    assert_eq!(decode(BRIGHTNESS, 1), Some(Event::Brightness(true)));
    assert_eq!(decode(BRIGHTNESS, 0), Some(Event::Brightness(false)));
    assert_eq!(decode(BRIGHTNESS, 2), None);
}

/// F-107: the Global Lock held (1) or being given back (0), nothing else.
#[test]
fn the_global_lock_report_decodes() {
    use super::platform_event::{decode, Event, GLOBAL_LOCK};
    assert_eq!(decode(GLOBAL_LOCK, 1), Some(Event::GlobalLock(true)));
    assert_eq!(decode(GLOBAL_LOCK, 0), Some(Event::GlobalLock(false)));
    assert_eq!(decode(GLOBAL_LOCK, 2), None);
}

/// F-108: a watch no capsule ever armed never acts on the clock alone;
/// _HOT and _CRT arm one, _PSV does not.
#[test]
fn a_watch_never_reported_never_acts() {
    use super::critical_watch::{Action, Watch, GRACE_MS};
    use super::platform_event::{CRITICAL, HOT, PASSIVE};
    use super::thermal_watch::alarming;
    let w = Watch::new();
    for now in [0, 1, GRACE_MS, 10 * GRACE_MS, u64::MAX] {
        assert_eq!(w.tick(now), Action::Nothing);
    }
    assert!(!alarming(PASSIVE | 3500) && alarming(HOT | 3700) && alarming(CRITICAL));
}
