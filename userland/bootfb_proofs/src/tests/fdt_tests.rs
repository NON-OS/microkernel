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

//! The aarch64 half (unit 14, C's record): a device-tree simple-framebuffer
//! node, bus 4 device 4, its bar the node's reg from the first pixel (aux 0),
//! brought up by the same steps and the same mode call as the x86 record;
//! a bus and device that do not belong together are left alone.

use super::firmware_model::Model;
use crate::bring_up::bring_up;
use crate::firmware::Why;
use crate::geometry::BGRX;
use crate::record::{BUS_KIND_FDT, FDT_FRAMEBUFFER, FIRMWARE_BOOT_FRAMEBUFFER};

#[test]
fn a_simple_framebuffer_node_is_brought_up_like_the_firmware_record() {
    let (mut m, mut r) = Model::new(64, 16, 256, BGRX, 0);
    (r.bus_kind, r.device) = (BUS_KIND_FDT, FDT_FRAMEBUFFER);
    let fb = bring_up(&mut m, &r).unwrap();
    assert_eq!(m.calls, ["announce", "claim", "geometry", "map"]);
    assert_eq!((fb.width, fb.height, fb.len as u64), (64, 16, r.bar0.3));
}

#[test]
fn a_bus_and_device_that_do_not_belong_together_are_left_alone() {
    let (mut m, mut r) = Model::new(64, 16, 256, BGRX, 0);
    (r.bus_kind, r.device) = (BUS_KIND_FDT, FIRMWARE_BOOT_FRAMEBUFFER);
    assert_eq!(bring_up(&mut m, &r).err(), Some(Why::Record));
    (r.bus_kind, r.device) = (5, FDT_FRAMEBUFFER);
    assert_eq!(bring_up(&mut m, &r).err(), Some(Why::Record));
    assert!(m.calls.is_empty());
}
