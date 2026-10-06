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

//! Once a flight has a verdict, nothing after it changes it.

use super::rfc8448_flight::{coalesced, keyed};
use crate::handshake_state::Progress;

#[test]
fn later_bytes_do_not_change_a_verdict() {
    let mut flight = coalesced();
    let mut state = keyed(&flight);
    let end = flight.len();
    assert_eq!(state.advance(&flight), Progress::Complete(end));
    flight.extend_from_slice(&[23, 3, 3, 0, 1, 0]);
    assert_eq!(state.advance(&flight), Progress::Complete(end));
}

#[test]
fn a_damaged_record_is_broken_not_incomplete() {
    let mut flight = coalesced();
    let at = flight.len() - 20;
    flight[at] ^= 1;
    assert_eq!(keyed(&flight).advance(&flight), Progress::Broken);
}
