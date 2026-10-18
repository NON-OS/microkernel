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

//! The zero-length report a pad rings for after resetting is held to the
//! same bounds as a mouse report: three set-ups with no touch report
//! between, a second apart at least.

use nonos_i2cmodel::touchpad::MODE_REPORT_ID;
use nonos_libc::mk_idle_ms;

use crate::drive::pass;
use crate::state::State;
use crate::tests::doorbell_tests::proven;
use crate::tests::wired::Wired;

/// The pad resets and rings at `now_ms`: whether it was set up again.
fn reset_and_ring(w: &Wired, s: &mut State, now_ms: u64) -> bool {
    w.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID, 0, 0]);
    w.held.set(true);
    pass(s, now_ms);
    w.held.set(false);
    w.pad.lock().feature(MODE_REPORT_ID) == Some(&[MODE_REPORT_ID, 3, 0b11][..])
}

#[test]
fn a_pad_that_keeps_resetting_is_set_up_three_times_and_no_more() {
    let (w, mut s) = proven();
    let tries: Vec<bool> = (1..=6)
        .map(|i| {
            mk_idle_ms(1_100);
            reset_and_ring(&w, &mut s, 1_000 + i * 1_100)
        })
        .collect();
    assert_eq!(tries, [true, true, true, false, false, false]);
}

#[test]
fn a_second_reset_within_the_holdoff_waits_for_it() {
    let (w, mut s) = proven();
    assert!(reset_and_ring(&w, &mut s, 1_010));
    assert!(!reset_and_ring(&w, &mut s, 1_030), "set up twice within a second");
    mk_idle_ms(1_100);
    assert!(reset_and_ring(&w, &mut s, 2_200), "not set up once the second had passed");
}
