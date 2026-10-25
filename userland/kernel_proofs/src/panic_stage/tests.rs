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

//! The stage rule: who takes the panic path, and what a second entry does.

use super::helpers::enter;
use super::stage::{next, word, Next, IDLE, STOPPING, WIPING};

#[test]
fn the_first_cpu_takes_the_path_and_names_itself() {
    let mut s = IDLE;
    assert_eq!(enter(&mut s, 3), Next::Full);
    assert_eq!(s, word(STOPPING, 3));
}

#[test]
fn a_fault_by_the_owner_before_its_wipe_goes_to_the_wipe() {
    let mut s = IDLE;
    enter(&mut s, 3);
    assert_eq!(enter(&mut s, 3), Next::Wipe);
}

#[test]
fn another_cpu_parks_for_the_owner_to_stop() {
    let mut s = IDLE;
    enter(&mut s, 3);
    assert_eq!(enter(&mut s, 4), Next::Park);
    assert_eq!(s, word(STOPPING, 3), "a parked cpu leaves the word as it was");
}

#[test]
fn once_the_wipe_began_everyone_parks_the_owner_too() {
    for owner in 1..=8 {
        for me in 1..=8 {
            assert_eq!(next(word(WIPING, owner), me), Next::Park, "{owner} {me}");
        }
    }
}
