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

//! F-103: a level GPE that never quiets and an EC GPE that fires without
//! end are turned off past the storm bound, told once; a sane rate never
//! is; and the power button is reported in every SCI, storm or none.

use super::gpe_storm_rig::storm;
use crate::events::{RUNS, WINDOW_MS};

#[test]
fn a_level_gpe_that_never_quiets_is_turned_off_and_the_button_still_reaches() {
    let (runs, stormed, pressed) = storm(0x01, 5_000, 1, false);
    assert_eq!(runs, RUNS as usize, "the bound's worth, then never again");
    assert_eq!(stormed, Some(u64::from(RUNS)));
    assert_eq!(pressed, 5_000, "the button in every SCI, storm or none");
}

#[test]
fn an_ec_gpe_firing_without_end_is_turned_off_too() {
    let (runs, stormed, pressed) = storm(0x17, 2_000, 0, true);
    assert_eq!((runs, stormed.is_some(), pressed), (RUNS as usize, true, 2_000));
}

#[test]
fn a_sane_rate_never_trips_the_bound() {
    let step = 2 * WINDOW_MS / u64::from(RUNS); // half the bound's rate
    let (runs, stormed, _) = storm(0x02, 3_000, step, false);
    assert_eq!((runs, stormed), (3_000, None));
}
