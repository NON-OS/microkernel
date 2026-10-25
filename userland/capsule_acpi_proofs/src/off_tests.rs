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

//! The wiped power-off's press rule (F-88): a press runs it only past the
//! boot guard and outside the debounce; a status bit stuck on never does.

use crate::off_rule::*;

#[path = "../../capsule_acpi/src/backlight/engine_rule.rs"]
mod engine_rule;
#[path = "off_levels_tests.rs"]
mod levels;
#[path = "off_owner_tests.rs"]
mod owner;
#[path = "off_restart_tests.rs"]
mod restart;

/// F-96: firmware's _BCM drives a panel only when the broker says nobody
/// holds its GPU; an engine that holds it, or a GPU that cannot be told,
/// keeps firmware off the backlight.
#[test]
fn bcm_only_for_a_panel_no_engine_drives() {
    assert!(engine_rule::bcm_allowed(Some(false)));
    assert!(!engine_rule::bcm_allowed(Some(true)));
    assert!(!engine_rule::bcm_allowed(None));
}

#[test]
fn a_press_powers_off_only_past_the_guard_and_the_bounce() {
    for pass in [None, Some(0u64), Some(1_000), Some(60_000)] {
        for last in [None, Some(0u64), Some(2_500), Some(9_700)] {
            for now in 0..12_000u64 {
                let bounced = last.is_some_and(|l| now.saturating_sub(l) < DEBOUNCE_MS);
                let guarded = pass.is_none_or(|t| now.saturating_sub(t) < BOOT_GUARD_MS);
                let want = match (bounced, guarded) {
                    (true, _) => Press::Bounce,
                    (false, true) => Press::BootGuard,
                    (false, false) => Press::PowerOff,
                };
                assert_eq!(press(last, pass, now), want, "{last:?} {pass:?} {now}");
            }
        }
    }
    assert_eq!(press(Some(1_000), Some(0), 1_499), Press::Bounce);
    assert_eq!(press(Some(1_000), Some(0), 2_000), Press::PowerOff, "500 ms on, 2 s in");
    assert_eq!(press(None, Some(5_000), 6_999), Press::BootGuard);
}

#[test]
fn a_stuck_event_never_powers_off() {
    // Each SCI 100 ms apart reports the press again: each is a bounce.
    let mut last = None;
    for now in (0..120_000u64).step_by(100) {
        let v = press(last, Some(0), now);
        assert_ne!(v, Press::PowerOff, "at {now}");
        last = Some(now);
    }
    // Once it stops, a real press past the debounce is taken.
    assert_eq!(press(last, Some(0), 120_000 + DEBOUNCE_MS), Press::PowerOff);
}
