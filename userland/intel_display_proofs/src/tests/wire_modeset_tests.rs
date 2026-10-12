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

//! MODE_SET on a taken-over DisplayPort output: offered as a list device,
//! and lit at the monitor's timing on new surfaces filled from the primary.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::body::caps::{MODE_RANGE, MODE_SET};
use nonos_gfx_wire::body::mode::{CURRENT, FROM_EDID};
use nonos_gfx_wire::Mode;

use super::dp_rig::dp_rig;
use super::edid_blob::edid_1080p_and_xga;
use super::wire_rig::h0;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::timing;
use crate::model::firmware::T1080;

pub const TOTAL: u32 = 1 << 20;

pub fn shadow_of(px: u32, w: u32, h: u32) -> Vec<u8> {
    px.to_le_bytes().iter().copied().cycle().take((w * h * 4) as usize).collect()
}

#[test]
fn a_dp_monitor_is_offered_its_listed_modes() {
    let (d, _, _) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), TOTAL);
    assert!(h0(&d).modeset.is_some(), "mode setting found");
    let c = d.caps();
    assert_eq!((c.flags & MODE_SET, c.flags & MODE_RANGE), (MODE_SET, 0), "a list device");
    let mut out = [Mode::default(); 4];
    let n = d.modes(0, &mut out).unwrap();
    let seen: Vec<_> =
        out[..n].iter().map(|m| (m.width, m.height, m.flags & (CURRENT | FROM_EDID))).collect();
    assert!(seen.contains(&(1024, 768, FROM_EDID)), "{seen:?}");
    assert!(seen.contains(&(1920, 1080, CURRENT | FROM_EDID)), "{seen:?}");
}

#[test]
fn a_mode_set_lights_the_new_timing_on_the_new_primary() {
    let (mut d, model, _) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), TOTAL);
    let px = shadow_of(0xFF13_5791, 1024, 768);
    let s = Shadow { bytes: &px, width: 1024, height: 768, stride: 4096, cursor_offset: 0 };
    d.set_mode(0, 1024, 768, &s).unwrap();
    let mut m = model.borrow_mut();
    let t = timing::read(&mut *m, Transcoder::B).unwrap();
    assert_eq!((t.h_active, t.v_active), (1024, 768));
    assert_eq!((h0(&d).scan.width, h0(&d).scan.height), (1024, 768));
    assert_eq!(m.pixel(Pipe::B, 1023, 767), Some(0xFF13_5791), "the new primary shows");
    assert_eq!(h0(&d).frames.was, None, "the old surfaces given back");
    assert_eq!(h0(&d).bases, h0(&d).frames.iova);
}
