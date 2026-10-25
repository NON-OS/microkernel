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

//! Findings about the kernel, each fixed in it and kept here as undo checks.
//! Two the model made. A step that read the clock once for all its jobs
//! dated a later job's window from before an earlier job's settle had
//! scrubbed its kept frames (I3): the kernel now reads it per job and stamps
//! each write once made, as `StallFresh` does. And the kept frames' bound
//! counted runs still waiting for their wipe and was asked before a lost
//! function was asked again, so one whose tenancy left 256 MiB was refused
//! for good once back: the bound now counts frames kept for good and stops a
//! new tenancy only, and I8 is judged at any bound. Three the L78 review
//! made, fixed by L80: a claim that read the bound before the record's hold,
//! or an end written before its frames were settled, let a tenancy in past
//! the bound (I10); and libc's 250 tries ran out with a reset under way (I9).

use super::model_mutation_tests::breaks;
use super::model_sweep::{sweep, METHODS};
use super::world_cases::CAP_LOST;
use super::world_knob::Knob;

#[test]
fn a_step_reads_the_clock_for_each_job() {
    breaks(Knob::Stall, 3);
    for m in METHODS {
        let s = sweep(m, Knob::StallFresh, 5_000, 0);
        assert_eq!(s.broke, 0, "{m:?}, the clock read again: {:?}", s.first);
    }
}

#[test]
fn a_lost_function_back_past_its_hold_is_asked_again_whatever_the_bound() {
    for m in METHODS {
        let s = sweep(m, Knob::Built, 2_000, 0);
        assert_eq!((s.broke, s.seen[CAP_LOST]), (0, 0), "{m:?}: {:?}", s.first);
    }
}

#[test]
fn the_kept_bound_read_before_the_hold_lets_a_tenancy_past_it() {
    breaks(Knob::CapEarly, 10);
}

#[test]
fn an_end_written_before_its_frames_are_settled_lets_a_tenancy_past_the_bound() {
    breaks(Knob::SettleLate, 10);
}

#[test]
fn libc_counting_250_tries_gives_up_on_a_reset_still_under_way() {
    breaks(Knob::FewTries, 9);
}
