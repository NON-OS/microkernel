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

//! A MODE_SET that cannot change anything is refused before the output
//! goes dark: a size the monitor does not list, no room for new surfaces.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::errno::{EINVAL, ENOMEM};

use super::dp_rig::dp_rig;
use super::edid_blob::edid_1080p_and_xga;
use super::wire_modeset_tests::{shadow_of, TOTAL};
use super::wire_rig::{h0, h0m};
use crate::engine::pipe::Pipe;
use crate::model::firmware::T1080;

#[test]
fn a_size_the_monitor_does_not_list_or_no_room_changes_nothing() {
    let (mut d, model, _) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), TOTAL);
    let px = shadow_of(0, 800, 600);
    let s = Shadow { bytes: &px, width: 800, height: 600, stride: 3200, cursor_offset: 0 };
    assert_eq!(d.set_mode(0, 800, 600, &s), Err(EINVAL));
    h0m(&mut d).frames.next = None;
    let px = shadow_of(0, 1024, 768);
    let s = Shadow { bytes: &px, width: 1024, height: 768, stride: 4096, cursor_offset: 0 };
    assert_eq!(d.set_mode(0, 1024, 768, &s), Err(ENOMEM));
    let m = model.borrow();
    assert_eq!(m.pixel(Pipe::B, 5, 5), Some(0xFF00_00AA), "never darkened");
    assert_eq!(h0(&d).scan.width, 1920);
}
