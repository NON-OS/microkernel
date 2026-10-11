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

//! Takeovers the engine refuses, before touching the plane.

use super::scanout_tests::{setup, TOTAL};
use crate::engine::error::Error;
use crate::engine::pipe::Pipe;
use crate::engine::scanout::Scanout;
use crate::engine::takeover::read_output;
use crate::model::Polls;

#[test]
fn a_tiled_firmware_plane_or_a_short_buffer_is_refused_untouched() {
    let (mut m, a, b) = setup();
    let mut out = read_output(&mut m, Pipe::A).unwrap();
    let short = &a[..2024];
    assert_eq!(
        Scanout::start(&mut m, &out, TOTAL, [short, &b], &Polls::new(9)).err(),
        Some(Error::Argument)
    );
    out.plane.ctl |= 1 << 10;
    assert_eq!(
        Scanout::start(&mut m, &out, TOTAL, [&a, &b], &Polls::new(9)).err(),
        Some(Error::Layout)
    );
    assert!(
        m.writes.iter().all(|w| w.0 == crate::engine::ggtt::GFX_FLSH_CNTL),
        "only setup's flush"
    );
}

#[test]
fn a_table_too_small_for_two_surfaces_is_full() {
    let (mut m, a, b) = setup();
    let out = read_output(&mut m, Pipe::A).unwrap();
    let r = Scanout::start(&mut m, &out, 4096, [&a, &b], &Polls::new(9));
    assert_eq!(r.err(), Some(Error::GgttFull));
}
