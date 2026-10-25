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

//! What bga answers on the display wire about itself: CAPS and MODES
//! (section 12), from the adapter's registers and the monitor's EDID.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::body::caps::{HW_CURSOR, MODE_RANGE, MODE_SET};
use nonos_gfx_wire::body::mode::{CURRENT, FROM_DEVICE, FROM_EDID, PREFERRED};
use nonos_gfx_wire::{kind, Mode};

use crate::dispi::DispiMode;
use crate::display::{fits, Bga};
use crate::edid::Panel;
use crate::model::{blob, dtd, Adapter, EST_1024X768, VRAM};

fn bga(a: &mut Adapter, edid: Option<[u8; 128]>, w: u32, h: u32) -> Bga {
    let panel = edid.map_or_else(Panel::default, |b| Panel::read(&b, |w, h| fits(VRAM, w, h)));
    a.bga(panel, DispiMode { width: w, height: h, stride: w * 4 })
}

fn modes(b: &Bga) -> Vec<(u32, u32, u32)> {
    let mut out = [Mode::default(); 20];
    let n = b.modes(0, &mut out).unwrap();
    out[..n].iter().map(|m| (m.width, m.height, m.flags)).collect()
}

#[test]
fn caps_name_a_dispi_device_that_sets_any_mode_in_range_and_has_no_cursor() {
    let mut a = Adapter::new();
    let c = bga(&mut a, Some(blob(Some(dtd(1280, 800, (286, 179))), &[], 0)), 1024, 768).caps();
    assert_eq!((c.kind, c.outputs, c.flags), (kind::DISPI, 1, MODE_SET | MODE_RANGE));
    assert_eq!(c.flags & HW_CURSOR, 0);
    assert_eq!((c.surface_budget, c.physical_mm), (VRAM as u64, 286 | 179 << 16));
    assert!(fits(VRAM, c.max_width, 480) && c.width_align == 8);
}

#[test]
fn with_no_edid_the_mode_shown_is_the_one_mode_and_preferred() {
    let mut a = Adapter::new();
    let all = CURRENT | FROM_DEVICE | PREFERRED;
    assert_eq!(modes(&bga(&mut a, None, 800, 600)), [(800, 600, all)]);
}

#[test]
fn edid_modes_are_offered_with_the_shown_one_marked_current() {
    let mut a = Adapter::new();
    let edid = blob(Some(dtd(1280, 800, (286, 179))), &[EST_1024X768], 0);
    let list = modes(&bga(&mut a, Some(edid), 1024, 768));
    assert!(list.contains(&(1280, 800, FROM_EDID | PREFERRED)), "{list:?}");
    assert!(list.contains(&(1024, 768, FROM_EDID | CURRENT | FROM_DEVICE)), "{list:?}");
    assert_eq!(list.len(), 2);
}

#[test]
fn a_shown_mode_the_monitor_does_not_list_is_still_offered() {
    let mut a = Adapter::new();
    let edid = blob(Some(dtd(1280, 800, (286, 179))), &[], 0);
    let list = modes(&bga(&mut a, Some(edid), 800, 600));
    assert_eq!(list[1], (800, 600, CURRENT | FROM_DEVICE), "{list:?}");
}
