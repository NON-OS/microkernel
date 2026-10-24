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

//! Flips: the old surface stays on screen until the latch, and the flip
//! returns only once the plane scans out the new one.

use crate::engine::error::Error;
use crate::engine::flip::flip;
use crate::engine::ggtt;
use crate::engine::ggtt_pte::Pte;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::plane_regs as r;
use crate::model::firmware::T1080;
use crate::model::{Gen9, Polls};

const FRONT: u32 = 0x4000_0000;
const BACK: u32 = 0x4080_0000;

fn two_buffers() -> Gen9 {
    let mut m = Gen9::new(0x80000);
    m.light(Transcoder::A, Pipe::A, T1080, 0);
    m.fill(0x100_0000, 1, 0xFFAA_0000);
    m.fill(0x200_0000, 1, 0xFF00_00BB);
    ggtt::map(&mut m, Pte::Wide, FRONT / 4096, &[0x100_0000; 2025], 0x80000).unwrap();
    ggtt::map(&mut m, Pte::Wide, BACK / 4096, &[0x200_0000; 2025], 0x80000).unwrap();
    m.set(r::plane_surf(Pipe::A, 1), FRONT);
    m.set(r::plane_surflive(Pipe::A, 1), FRONT);
    m
}

#[test]
fn a_flip_returns_once_the_back_buffer_is_live() {
    let mut m = two_buffers();
    assert_eq!(m.pixel(Pipe::A, 0, 0), Some(0xFFAA_0000));
    flip(&mut m, Pipe::A, BACK, &Polls::new(100)).unwrap();
    assert_eq!(m.get(r::plane_surflive(Pipe::A, 1)), BACK);
    assert_eq!(m.pixel(Pipe::A, 100, 100), Some(0xFF00_00BB));
    flip(&mut m, Pipe::A, FRONT, &Polls::new(100)).unwrap();
    assert_eq!(m.pixel(Pipe::A, 100, 100), Some(0xFFAA_0000));
}

#[test]
fn a_pipe_that_never_blanks_times_out_with_the_old_surface_shown() {
    let mut m = two_buffers();
    m.period = 0;
    assert_eq!(flip(&mut m, Pipe::A, BACK, &Polls::new(50)), Err(Error::Timeout));
    assert_eq!(m.pixel(Pipe::A, 0, 0), Some(0xFFAA_0000));
}

#[test]
fn a_misaligned_surface_is_refused_before_any_write() {
    let mut m = two_buffers();
    m.writes.clear();
    assert_eq!(flip(&mut m, Pipe::A, BACK + 0x1000, &Polls::new(9)), Err(Error::Argument));
    assert!(m.writes.is_empty());
}
