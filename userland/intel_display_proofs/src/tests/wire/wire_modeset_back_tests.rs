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

//! A mode set that fails after the output went dark puts the old mode back
//! on the old surfaces, and says why; and where mode setting cannot be had,
//! it is not offered.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::errno::ENOMEM;

use crate::display::{Modeset, Monitor, EDID_MAX};
use crate::engine::gen::{GEN11, GEN9};
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::timing;
use crate::model::firmware::Axis;
use crate::model::Ticks;
use crate::tests::dp_rig::dp_rig;
use crate::tests::edid_blob::edid_1080p_and_xga;
use crate::tests::wire_rig::{h0, rig};

/// 1024x768 at 60 Hz, the DMT's timing.
const T768: (Axis, Axis) = ((1024, 24, 136, 1344), (768, 3, 6, 806));

#[test]
fn a_new_mode_with_no_room_in_the_ggtt_goes_back_to_the_old_one() {
    /*
     * 4096 entries hold two 1024x768 surfaces beside firmware's, not two
     * 1920x1080 ones: the new surfaces fail to map after the port was lit.
     */
    let (mut d, model, _) = dp_rig(T768, edid_1080p_and_xga((600, 340)), 4096);
    let px: Vec<u8> = vec![0x77; 1920 * 1080 * 4];
    let s = Shadow { bytes: &px, width: 1920, height: 1080, stride: 7680, cursor_offset: 0 };
    assert_eq!(d.set_mode(0, 1920, 1080, &s), Err(ENOMEM));
    let mut m = model.borrow_mut();
    let t = timing::read(&mut *m, Transcoder::B).unwrap();
    assert_eq!((t.h_active, t.v_active), (1024, 768), "the old timing back");
    assert_eq!(m.pixel(Pipe::B, 1023, 767), Some(0xFF00_00AA), "the old picture back");
    assert_eq!((h0(&d).scan.width, h0(&d).frames.was), (1024, None));
}

#[test]
fn edp_another_generation_or_a_silent_monitor_has_no_mode_setting() {
    let (mut d, _) = rig();
    let clock = Ticks::new();
    let mon = Monitor::of(&mut d.m, &clock, &GEN9, Transcoder::Edp);
    let out = h0(&d).out;
    assert!(Modeset::find(&mut d.m, &clock, &GEN9, &out, &mon, (0, 1 << 20)).is_none(), "eDP");
    let (mut d, _, _) = dp_rig(T768, edid_1080p_and_xga((600, 340)), 1 << 20);
    let mon = Monitor::of(&mut d.m, &clock, &GEN9, Transcoder::B);
    let out = h0(&d).out;
    assert!(Modeset::find(&mut d.m, &clock, &GEN11, &out, &mon, (0, 1 << 20)).is_none());
    let silent = Monitor { blob: [0; EDID_MAX], len: 0 };
    assert!(Modeset::find(&mut d.m, &clock, &GEN9, &out, &silent, (0, 1 << 20)).is_none());
}
