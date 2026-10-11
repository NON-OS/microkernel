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

//! Firmware states a takeover reads but must not trust.

use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::takeover::read_output;
use crate::model::firmware::T1080;
use crate::model::Gen9;

#[test]
fn a_timing_with_sync_outside_its_blanking_is_refused() {
    let mut m = Gen9::new(64);
    let bad = ((1920, 300, 44, 2200), (1080, 4, 5, 1125));
    m.light(Transcoder::A, Pipe::A, bad, 0);
    assert!(read_output(&mut m, Pipe::A).is_none());
}

#[test]
fn a_tiled_plane_is_lit_but_not_flippable() {
    let mut m = Gen9::new(64);
    m.light(Transcoder::A, Pipe::A, T1080, 0);
    m.set(crate::engine::plane_regs::plane_ctl(Pipe::A, 1), (1 << 31) | (4 << 24) | (1 << 10));
    let o = read_output(&mut m, Pipe::A).unwrap();
    assert!(o.plane.enabled() && !o.plane.flippable());
}
