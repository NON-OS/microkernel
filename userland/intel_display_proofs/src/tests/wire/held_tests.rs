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

//! Two lit outputs, B's monitor pulled, a mode set on A while A's plane is
//! the only one lit, then B plugged back: A keeps its own runs, since B is
//! still held and its runs are kept for it, so B comes back to runs that
//! meet no other's. Taking the whole buffer for A would have left B's old
//! runs overlapping A's, or refused B for good.

use crate::engine::ddb::Run;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::plane_read::ENABLE;
use crate::engine::plane_regs::plane_ctl;
use crate::engine::relight::buffer;
use crate::engine::timing;
use crate::tests::dp_rig::Model;
use crate::tests::two_rig::two_rig;
use crate::tests::wire_rig::rig;

use super::share_rig::{beside, runs, A, B};

const LATENCY: [u32; 8] = [2, 4, 8, 16, 24, 32, 48, 0];

fn meets(a: Run, b: Run) -> bool {
    a.start < b.end && b.start < a.end
}

/// Relight `p`'s buffer with B's plane lit or not, `held` as the driver says.
fn relight(model: &Model, p: Pipe, b_lit: bool, held: bool) {
    let mut m = model.borrow_mut();
    m.set(plane_ctl(Pipe::B, 1), if b_lit { ENABLE } else { 0 });
    let t = timing::read(&mut *m, Transcoder::B).unwrap();
    buffer(&mut *m, p, 1920, &t, &LATENCY, held).unwrap();
}

#[test]
fn a_pulled_output_keeps_its_runs_through_a_mode_set_beside_it() {
    let (_d, model) = beside(B);
    relight(&model, Pipe::A, false, true);
    assert_eq!(runs(&model, Pipe::A).0, A, "B held: A keeps its own");
    relight(&model, Pipe::B, true, true);
    let (a, b) = (runs(&model, Pipe::A), runs(&model, Pipe::B));
    assert_eq!(b, B, "B back on its own runs");
    assert!(!meets(a.0, b.0) && !meets(a.0, b.1), "no overlap");
}

#[test]
fn not_held_and_no_other_plane_lit_the_pipe_takes_the_whole_buffer() {
    let (_d, model) = beside(B);
    model.borrow_mut().set(plane_ctl(Pipe::A, 1), 0);
    relight(&model, Pipe::B, true, false);
    assert_eq!(runs(&model, Pipe::B).0, Run { start: 0, end: 864 });
}

#[test]
fn another_head_is_held_beside_lit_or_pulled() {
    let (two, _, _) = two_rig();
    assert!(two.held_beside(0) && two.held_beside(1));
    assert!(!rig().0.held_beside(0), "alone");
}
