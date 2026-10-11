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

//! Power well requests and which wells an output needs.

use crate::engine::error::Error;
use crate::engine::pipe::Transcoder;
use crate::engine::power::{enable, needed, request_bit, Well, PWR_WELL_CTL2};
use crate::model::{Gen9, Polls};

#[test]
fn a_well_is_requested_and_waited_for() {
    let mut m = Gen9::new(1);
    enable(&mut m, Well::Two, &Polls::new(20)).unwrap();
    assert_eq!(m.get(PWR_WELL_CTL2) & (3 << 30), 3 << 30);
    enable(&mut m, Well::DdiIo(1), &Polls::new(20)).unwrap();
    assert_eq!(m.get(PWR_WELL_CTL2) & (3 << 4), 3 << 4, "DDI B's request and state");
    assert_eq!(m.get(PWR_WELL_CTL2) & (3 << 30), 3 << 30, "well 2 left on");
}

#[test]
fn a_well_that_never_comes_up_times_out() {
    let mut m = Gen9::new(1);
    m.dead = true;
    assert_eq!(enable(&mut m, Well::Two, &Polls::new(20)), Err(Error::Timeout));
}

#[test]
fn request_bits_and_the_wells_each_output_needs() {
    assert_eq!(request_bit(Well::DdiIo(0)), 1 << 3);
    assert_eq!(request_bit(Well::DdiIo(3)), 1 << 9);
    assert_eq!(needed(Transcoder::Edp, 0), [None, Some(Well::DdiIo(0))]);
    assert_eq!(needed(Transcoder::A, 1), [Some(Well::Two), Some(Well::DdiIo(1))]);
    assert_eq!(needed(Transcoder::Edp, 2), [Some(Well::Two), Some(Well::DdiIo(2))]);
}
