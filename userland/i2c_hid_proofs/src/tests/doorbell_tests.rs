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

//! Reads paced by the pad's interrupt line: none while it is quiet; the
//! zero-length report it rings for after resetting itself sets the pad up
//! again; and a line that rings with nothing to read is given up.

use nonos_i2cmodel::touchpad::{touch_report, Touch, MODE_REPORT_ID};
use nonos_i2cmodel::Command;

use super::fixture::pad;
use super::wired::{wired, Wired};
use crate::drive::pass;
use crate::setup;
use crate::state::State;

/// Bound, and the line proven by one report it rang for.
pub(super) fn proven() -> (Wired, State) {
    let w = wired(pad());
    let mut s = setup::run().expect("setup");
    w.pad.lock().push_input(&touch_report(&Touch::finger(100, 100)));
    pass(&mut s, 1_000);
    assert!(s.doorbell.proven, "the line never rang");
    w.pad.lock().take_commands();
    (w, s)
}

fn reads(w: &Wired) -> usize {
    let read = |c: &&Command| matches!(c, Command::InputRead | Command::RegisterRead(3));
    w.pad.lock().commands().iter().filter(read).count()
}

#[test]
fn a_quiet_trusted_line_means_no_read() {
    let (w, mut s) = proven();
    (1_002..1_500).for_each(|t| pass(&mut s, t));
    assert_eq!(reads(&w), 0, "the pad was read with nothing to read");
}

#[test]
fn a_zero_length_report_rung_for_sets_the_pad_up_again() {
    let (w, mut s) = proven();
    w.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID, 0, 0]);
    w.held.set(true);
    pass(&mut s, 1_010);
    w.held.set(false);
    assert_eq!(w.pad.lock().feature(MODE_REPORT_ID), Some(&[MODE_REPORT_ID, 3, 0b11][..]));
}

#[test]
fn a_line_that_rings_with_nothing_to_read_is_given_up_after_sixteen() {
    let (w, mut s) = proven();
    w.held.set(true);
    (0..40).for_each(|i| pass(&mut s, 2_000 + i * 20));
    assert!(s.doorbell.absent && !s.doorbell.proven, "a stuck line is still trusted");
    let sets =
        w.pad.lock().commands().iter().filter(|c| matches!(c, Command::SetReport { .. })).count();
    assert!(sets <= 3, "the pad was set up {sets} times for one stuck line");
}
