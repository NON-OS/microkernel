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

//! The undo checks of plan section 6 the seeded model owns (1, 4, 5, 6, 8, 9,
//! 10, 13 and 14): with its knob, some seed of some method breaks the one
//! invariant named, where the model as built breaks none (model_tests).

use super::model_sweep::{sweep, METHODS};
use super::world_knob::Knob;

/// The knob breaks invariant `inv` in some seed of some method.
pub(super) fn breaks(knob: Knob, inv: u8) {
    let broke = METHODS.iter().any(|&m| sweep(m, knob, 5_000, 1 << inv).broke & (1 << inv) != 0);
    assert!(broke, "{knob:?} broke no I{inv}");
}

#[test]
fn frames_freed_at_teardown_are_reached_by_a_device_still_mastering() {
    breaks(Knob::FreeAtTeardown, 2);
}

#[test]
fn a_settle_without_its_epoch_frees_a_newer_holders_frames() {
    breaks(Knob::SettleNoEpoch, 2);
}

#[test]
fn a_vendor_id_read_at_99_ms_falls_inside_the_flr_window() {
    breaks(Knob::VendorAt99, 3);
}

#[test]
fn a_display_reset_trusted_lets_another_service_into_its_memory() {
    breaks(Knob::DisplayTrusted, 1);
}

#[test]
fn frames_freed_from_a_function_past_bus_master_are_reached() {
    breaks(Knob::FreeWhenBinds, 2);
}

#[test]
fn an_answer_taken_without_its_marker_trusts_an_ignored_reset() {
    breaks(Knob::NoMarker, 1);
}

#[test]
fn without_the_busy_mark_two_steppers_write_one_pass_twice() {
    breaks(Knob::NoToken, 4);
}

#[test]
fn a_lost_function_never_asked_again_stays_refused_once_back() {
    breaks(Knob::NoRearm, 8);
}

#[test]
fn a_reset_written_over_pending_transactions() {
    breaks(Knob::NoTpWait, 4);
}
