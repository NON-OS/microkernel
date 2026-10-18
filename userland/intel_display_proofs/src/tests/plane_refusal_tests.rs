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

//! A plane program the engine cannot take changes nothing.

use super::plane_tests::{lit, OURS};
use crate::engine::error::Error;
use crate::engine::gen::PlaneKind;
use crate::engine::pipe::Pipe;
use crate::engine::plane::program;

#[test]
fn a_bad_address_stride_or_size_writes_nothing() {
    let mut m = lit();
    m.writes.clear();
    let ctl = 1 << 31;
    let cases = [
        (OURS + 4096, 7680, (1920, 1080)),
        (OURS, 7679, (1920, 1080)),
        (OURS, 7616, (1920, 1080)),
        (OURS, 1024 * 64, (1920, 1080)),
        (OURS, 7680, (0, 1080)),
        (OURS, 7680, (1920, 4097)),
    ];
    for (surf, stride, size) in cases {
        assert_eq!(
            program(&mut m, PlaneKind::Universal, Pipe::A, surf, stride, size, ctl),
            Err(Error::Argument)
        );
    }
    assert!(m.writes.is_empty());
}
