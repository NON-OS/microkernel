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

//! An RMRR is resolved at its own size, eight scopes, never a remapping
//! unit's 255, and a count past its array is held to the array: the scope
//! list a claim walks never reads past what the region holds.

use super::dmar_rmrr::{Rmrr, MAX_RMRR_SCOPES};
use super::dmar_rmrr_names::rmrr_names;
use super::scope::{Cover, Scope, SCOPE_ENDPOINT};
use super::tests::bridges;

fn region(count: u8) -> Rmrr {
    let mut scopes = [Scope::EMPTY; MAX_RMRR_SCOPES];
    for (i, s) in scopes.iter_mut().enumerate() {
        *s = Scope { kind: SCOPE_ENDPOINT, start_bus: 0, path: [(i as u8, 0); 4], path_len: 1 };
    }
    Rmrr {
        segment: 0,
        base: 0x1000,
        limit: 0x1fff,
        scopes,
        count,
        truncated: false,
        foreign: false,
    }
}

#[test]
fn the_eighth_scope_is_read() {
    assert_eq!(rmrr_names(&region(8), 0, (0, 7, 0), &bridges), Cover::Yes);
    assert_eq!(rmrr_names(&region(7), 0, (0, 7, 0), &bridges), Cover::No);
}

#[test]
fn a_count_past_the_array_reads_the_array_and_no_further() {
    assert_eq!(rmrr_names(&region(200), 0, (0, 7, 0), &bridges), Cover::Yes);
    assert_eq!(rmrr_names(&region(200), 0, (0, 8, 0), &bridges), Cover::No);
}
