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

use super::lid_state::{lid_open, record};
use super::platform_event::{decode, Event, CRITICAL, HOT, LID, POWER_BUTTON, THERMAL};
use super::thermal_watch::{latest, thermal, Action, GRACE_MS};

#[test]
fn events_decode_and_stray_bits_are_refused() {
    assert_eq!(decode(POWER_BUTTON, 0), Some(Event::PowerButton));
    assert_eq!(decode(POWER_BUTTON, 1), None);
    assert_eq!(decode(LID, 1), Some(Event::Lid(true)));
    assert_eq!(decode(LID, 0), Some(Event::Lid(false)));
    assert_eq!(decode(LID, 2), None);
    assert_eq!(decode(THERMAL, CRITICAL | HOT | 3732), Some(Event::Thermal(CRITICAL | HOT | 3732)));
    assert_eq!(decode(THERMAL, 1 << 19), None);
    assert_eq!(decode(9, 0), None);
}

#[test]
fn a_critical_zone_asks_first_and_powers_off_after_the_grace() {
    assert_eq!(thermal(3500, 1_000), Action::Nothing);
    assert_eq!(latest(1_000), Some(3500));
    assert_eq!(thermal(CRITICAL | 3800, 2_000), Action::AskShutdown);
    assert_eq!(thermal(CRITICAL | 3800, 2_000 + GRACE_MS - 1), Action::Nothing);
    assert_eq!(thermal(CRITICAL | 3800, 2_000 + GRACE_MS), Action::PowerOff);
    assert_eq!(thermal(3000, 40_000), Action::Nothing, "cooled: the watch resets");
    assert_eq!(thermal(CRITICAL | 3800, 41_000), Action::AskShutdown, "a new episode asks again");
    assert_eq!(latest(41_000 + 180_000), Some(CRITICAL | 3800));
    assert_eq!(latest(41_000 + 180_001), None, "stale");
}

#[test]
fn the_lid_says_when_it_changes() {
    assert_eq!(lid_open(), None);
    assert!(record(false));
    assert!(!record(false), "unchanged");
    assert_eq!(lid_open(), Some(false));
    assert!(record(true));
    assert_eq!(lid_open(), Some(true));
}
