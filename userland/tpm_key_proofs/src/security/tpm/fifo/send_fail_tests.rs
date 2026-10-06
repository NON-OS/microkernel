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

//! Refusals while the command goes in or runs, or with nowhere to put the
//! answer.

use super::fail::FifoFail;
use super::harness::{frame, refused};
use super::model::Model;

#[test]
fn a_part_that_parses_a_shorter_command_is_refused() {
    let mut model = Model::new(frame(10, 0));
    model.parsed_len = Some(12);
    assert_eq!(refused(&mut model, &frame(16, 6), 32), FifoFail::ExpectDropped);
    assert_eq!(model.commands, 0);
}

#[test]
fn a_part_that_still_expects_after_the_last_byte_is_refused() {
    let mut model = Model::new(frame(10, 0));
    model.parsed_len = Some(20);
    assert_eq!(refused(&mut model, &frame(16, 6), 32), FifoFail::ExpectStillSet);
    assert_eq!(model.commands, 0);
}

#[test]
fn a_burst_count_stuck_at_zero_times_out() {
    let mut model = Model::new(frame(10, 0));
    model.burst = 0;
    assert_eq!(refused(&mut model, &frame(12, 2), 32), FifoFail::NoBurst);
    assert!(model.received.is_empty());
}

#[test]
fn a_part_that_never_answers_times_out() {
    let mut model = Model::new(frame(10, 0));
    model.exec_polls = u64::MAX;
    assert_eq!(refused(&mut model, &frame(12, 2), 32), FifoFail::NoResponse);
    assert_eq!(model.commands, 1);
}

#[test]
fn a_buffer_too_small_for_a_header_is_refused_after_the_command_ran() {
    let mut model = Model::new(frame(10, 0));
    assert_eq!(refused(&mut model, &frame(12, 2), 8), FifoFail::ResponseSize);
    assert_eq!(model.commands, 1);
}
