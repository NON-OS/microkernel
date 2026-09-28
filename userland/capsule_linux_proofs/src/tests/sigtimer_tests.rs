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

//! The arithmetic of ITIMER_REAL: a periodic timer moves past every period
//! that ended while it was not looked at, and a one-shot one stops.

use crate::sigtimer::Itimer;

#[test]
fn a_periodic_itimer_moves_past_every_period_that_ended() {
    let mut t = Itimer { due: 1000, interval: 200 };
    assert!(t.rearm(1450));
    assert_eq!(t.due, 1600);
}

#[test]
fn a_one_shot_itimer_does_not_fire_again() {
    let mut t = Itimer { due: 1000, interval: 0 };
    assert!(!t.rearm(1000));
}
