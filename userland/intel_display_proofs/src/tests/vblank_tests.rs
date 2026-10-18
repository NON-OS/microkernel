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

//! Waiting for a vertical blank on the frame counter.

use crate::engine::error::Error;
use crate::engine::pipe::Pipe;
use crate::engine::regs;
use crate::engine::vblank::wait_vblank;
use crate::model::{Gen9, Polls};

#[test]
fn the_wait_returns_after_exactly_one_step_of_the_counter() {
    let mut m = Gen9::new(1);
    m.set(regs::pipe_frmcnt(Pipe::B), 41);
    wait_vblank(&mut m, Pipe::B, &Polls::new(10)).unwrap();
    assert_eq!(m.get(regs::pipe_frmcnt(Pipe::B)), 42);
}

#[test]
fn a_counter_that_never_moves_times_out() {
    let mut m = Gen9::new(1);
    m.period = 0;
    assert_eq!(wait_vblank(&mut m, Pipe::C, &Polls::new(25)), Err(Error::Timeout));
}

#[test]
fn a_counter_wrapping_through_zero_still_counts() {
    let mut m = Gen9::new(1);
    m.set(regs::pipe_frmcnt(Pipe::A), u32::MAX);
    wait_vblank(&mut m, Pipe::A, &Polls::new(10)).unwrap();
    assert_eq!(m.get(regs::pipe_frmcnt(Pipe::A)), 0);
}
