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
//! Body step.

use crate::body_step::{step, Step};

#[test]
fn bytes_are_always_kept() {
    assert_eq!(step(1, false, false), Step::Keep);
    assert_eq!(step(16_384, true, true), Step::Keep, "a full chunk outranks both flags");
}
/*
 * The failure this exists for. A 370 kB consensus arrives in segments with gaps
 * between them, and the old rule read a run of empty reads as the end: four hundred
 * of them, each one IPC round trip and a yield, pass in well under a millisecond
 * while a real gap is milliseconds. Every large document ended early, failed to
 * inflate, and was reported as a body the authority would not serve.
 */
#[test]
fn a_gap_in_the_middle_is_not_the_end() {
    assert_eq!(
        step(0, false, false),
        Step::Wait,
        "quiet with the socket open is a pause, and the only answer is to wait"
    );
}
#[test]
fn a_closed_socket_drains_before_it_stops() {
    assert_eq!(step(0, true, false), Step::Drain);
}
