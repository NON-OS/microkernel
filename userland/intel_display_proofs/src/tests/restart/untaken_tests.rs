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
//! A lit output the restart does not take (G-78): on a killed driver's
//! leftover, its plane is turned off and its entries go with the rest; on
//! firmware's picture, it keeps showing it and firmware's entries stay.

use super::kit::{restart, TOTAL};
use super::leftovers::leftovers;
use super::restart_tests::{base, reaches_only_ours};
use crate::engine::gen::GEN9;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::plane::program;
use crate::engine::plane_regs as r;
use crate::model::firmware::Axis;
use crate::tests::confine::kit::confined;

const T768: (Axis, Axis) = ((1024, 24, 136, 1344), (768, 3, 6, 806));

fn with_pipe_b() -> crate::model::Gen9 {
    let mut m = base();
    m.light(Transcoder::B, Pipe::B, T768, (1 << 28) | (2 << 24) | (3 << 1));
    m
}

#[test]
fn an_untaken_output_on_a_leftover_goes_dark() {
    let mut m = with_pipe_b();
    leftovers(&mut m, Pipe::B, TOTAL - 16384, (1024, 768));
    let (_d, model) = restart(m).expect("lit");
    let m = model.borrow();
    assert_eq!(m.live_reg(r::plane_ctl(Pipe::B, 1)) >> 31, 0, "pipe B's plane off");
    confined(&m, "untaken leftover");
    reaches_only_ours(&m, "untaken leftover");
}

#[test]
fn an_untaken_output_on_firmware_s_picture_keeps_it() {
    let mut m = with_pipe_b();
    let ctl = m.get(r::plane_ctl(Pipe::B, 1));
    program(&mut m, GEN9.plane, Pipe::B, 0, 4096, (1024, 768), ctl).unwrap();
    m.vblank();
    let (_d, model) = restart(m).expect("lit");
    let m = model.borrow();
    assert_eq!(m.pixel(Pipe::B, 9, 9), Some(0xFF00_0001), "firmware's picture still");
    assert_eq!(m.live_reg(r::plane_ctl(Pipe::B, 1)) >> 31, 1);
    confined(&m, "untaken firmware");
    assert_eq!(m.pixel(Pipe::A, 5, 5), Some(0xFF00_00AA));
}

#[test]
fn firmware_s_plane_another_size_than_its_pipe_is_still_refused() {
    use crate::engine::{error::Error, scanout::Scanout, takeover::read_output};
    let mut m = base();
    m.set(crate::engine::regs::pipe_src(Pipe::A), (1023 << 16) | 767);
    let out = read_output(&mut m, &GEN9, Pipe::A).unwrap();
    let a: Vec<u64> = (0..768).map(|i| 0x5000_0000 + i * 4096).collect();
    let got = Scanout::start(&mut m, &out, TOTAL, [&a, &a], TOTAL / 2, &crate::model::Polls::new(50));
    assert_eq!(got.err(), Some(Error::Layout), "only a leftover is drawn over at another size");
}
