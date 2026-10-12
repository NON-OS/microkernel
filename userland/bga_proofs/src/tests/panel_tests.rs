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

//! What bga takes from the monitor's EDID: its preferred mode, its size,
//! and the modes it lists that DISPI can show in video memory, once each.

use nonos_gfx_wire::body::mode::{FROM_EDID, PREFERRED};

use crate::display::fits;
use crate::edid::Panel;
use crate::model::{blob, dtd, EST_1024X768, EST_640X480, EST_720X400, VRAM};

fn panel(b: &[u8]) -> Panel {
    Panel::read(b, |w, h| fits(VRAM, w, h))
}

fn listed(p: &Panel) -> Vec<(u32, u32, u32)> {
    p.modes[..p.len].iter().map(|m| (m.width, m.height, m.flags)).collect()
}

#[test]
fn the_first_detailed_timing_is_preferred_and_names_the_size() {
    let p = panel(&blob(Some(dtd(1280, 800, (286, 179))), &[], 0));
    assert_eq!(p.preferred, Some((1280, 800)));
    assert_eq!(p.physical_mm, 286 | 179 << 16);
    assert_eq!(listed(&p), [(1280, 800, FROM_EDID | PREFERRED)]);
}

#[test]
fn established_modes_are_listed_when_dispi_can_show_them() {
    /*
     * 720x400 is a text mode's timing: below section 12's 640x480 floor,
     * so it is not offered.
     */
    let est = [EST_720X400, EST_640X480, EST_1024X768];
    let p = panel(&blob(Some(dtd(1280, 800, (286, 179))), &est, 0));
    let sizes: Vec<_> = listed(&p).iter().map(|m| (m.0, m.1)).collect();
    assert!(sizes.contains(&(640, 480)) && sizes.contains(&(1024, 768)), "{sizes:?}");
    assert!(!sizes.contains(&(720, 400)), "{sizes:?}");
}

#[test]
fn a_mode_listed_twice_is_offered_once() {
    let p = panel(&blob(Some(dtd(1024, 768, (286, 179))), &[EST_1024X768], 0));
    assert_eq!(listed(&p), [(1024, 768, FROM_EDID | PREFERRED)]);
}

#[test]
fn a_preferred_mode_video_memory_cannot_hold_is_not_preferred() {
    let p = panel(&blob(Some(dtd(3840, 2160, (600, 340))), &[EST_1024X768], 0));
    assert_eq!(p.preferred, None);
    assert_eq!(listed(&p), [(1024, 768, FROM_EDID)]);
}

#[test]
fn a_blob_that_does_not_parse_is_an_empty_panel() {
    let mut b = blob(Some(dtd(1280, 800, (286, 179))), &[], 0);
    b[127] ^= 1;
    let p = panel(&b);
    assert_eq!((p.preferred, p.physical_mm, p.len), (None, 0, 0));
}
