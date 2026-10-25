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

//! The plane's checks and a mode set at every edge of width, height and
//! stride: taken exactly when the test's reading says, never a wrap.

use super::edges::{pairs, EDGES};
use crate::engine::error::Error;
use crate::engine::gen::PlaneKind;
use crate::engine::pipe::Pipe;
use crate::engine::plane::program;
use crate::model::firmware::T1080;
use crate::tests::dp_rig::dp_rig;
use crate::tests::edid_blob::edid_1080p_and_xga;
use crate::tests::plane_tests::{lit, OURS};

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::errno::EINVAL;

#[test]
fn a_plane_is_programmed_exactly_when_its_sizes_hold_at_every_edge() {
    let (mut m, kind) = (lit(), PlaneKind::Universal);
    let mut taken = 0;
    for ((w, h), stride) in pairs().flat_map(|p| EDGES.iter().map(move |&s| (p, s))) {
        let ok = stride % 64 == 0
            && stride as u128 >= w as u128 * 4
            && stride <= kind.max_stride()
            && (1..=8192).contains(&w)
            && (1..=4096).contains(&h);
        let got = program(&mut m, kind, Pipe::A, OURS, stride, (w, h), 1 << 31);
        assert_eq!(got, if ok { Ok(()) } else { Err(Error::Argument) }, "{w}x{h} {stride}");
        taken += ok as u32;
    }
    assert!(taken > 10, "{taken}");
}

#[test]
fn a_mode_set_to_any_edge_size_is_refused_before_a_product_is_taken() {
    let (mut d, _, _) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), 1 << 20);
    let px = vec![0u8; 64];
    let s = Shadow { bytes: &px, width: 4, height: 4, stride: 16, cursor_offset: 0 };
    for (w, h) in pairs() {
        assert_eq!(d.set_mode(0, w, h, &s), Err(EINVAL), "{w}x{h}");
    }
}
