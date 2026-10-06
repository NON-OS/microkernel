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

//! The shipping FIFO protocol against the model when the part behaves.

use super::fail::FifoFail;
use super::harness::{command, frame, run};
use super::model::{Model, State};

#[test]
fn a_command_goes_in_and_its_response_comes_out_at_every_burst() {
    for burst in 1..=40 {
        let response = frame(37, 27);
        let mut model = Model::new(response.clone());
        model.burst = burst;
        let cmd = command(23);
        let mut out = [0u8; 64];
        assert_eq!(run(&mut model, &cmd, &mut out), Ok(37), "burst {burst}");
        assert_eq!(&out[..37], &response[..]);
        assert_eq!(model.ran, cmd, "burst {burst}");
        assert_eq!(model.commands, 1);
    }
}

#[test]
fn back_to_back_commands_each_find_the_part_ready() {
    let mut model = Model::new(frame(10, 0));
    let mut out = [0u8; 16];
    for round in 1..=3 {
        model.exec_polls = 3;
        assert_eq!(run(&mut model, &command(12), &mut out), Ok(10));
        assert_eq!(model.commands, round);
        assert_eq!(model.state, State::Idle, "commandReady returned the part to idle");
    }
}

#[test]
fn a_part_that_never_grants_the_locality_is_refused() {
    let mut model = Model::new(frame(10, 0));
    model.grant = false;
    assert_eq!(run(&mut model, &command(12), &mut [0u8; 16]), Err(FifoFail::LocalityNotGranted));
    assert_eq!(model.commands, 0);
}

#[test]
fn a_command_shorter_than_its_header_is_never_sent() {
    let mut model = Model::new(frame(10, 0));
    let cmd = [0x80, 0x01, 0, 0, 0, 9, 0, 0, 1];
    assert_eq!(run(&mut model, &cmd, &mut [0u8; 16]), Err(FifoFail::CommandTooShort));
    assert!(model.received.is_empty());
}

#[test]
fn a_part_left_mid_command_is_reset_before_the_next() {
    let mut model = Model::new(frame(10, 0));
    model.state = State::Reception;
    model.received = vec![0x80, 0x01, 0, 0];
    assert_eq!(run(&mut model, &command(12), &mut [0u8; 16]), Ok(10));
    assert_eq!(model.ran, command(12));
}
