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

//! A stall once the primary surface is shared. Whatever the
//! device does next the driver keeps its claim and every grant, so the
//! frames the compositor draws into are never freed under it. A device that
//! works after a reset is brought back in place: the same resource on the
//! same frames, shown again, under the same surface handle. One that does
//! not leaves the driver up, `Down` by name and answering E_DEVICE.

use nonos_libc::{releases, said};
use nonos_virtio::common::DEVICE_STATUS;

use super::client::call;
use super::frames::le32;
use super::part::{answering_after_reset, Answer};
use super::stall::{handle, health, stalled, whole_flush, CLIENT, PRIMARY};
use crate::constants::{VG_CMD_RESOURCE_ATTACH_BACKING, VG_CMD_RESOURCE_CREATE_2D};
use crate::constants::{VG_CMD_RESOURCE_FLUSH, VG_CMD_SET_SCANOUT, VG_CMD_TRANSFER_TO_HOST_2D};
use crate::protocol::{E_DEVICE, OP_FLUSH};
use crate::server::recover::settle;
use crate::state::Health;

#[test]
fn a_stall_then_a_working_device_is_recovered_on_the_same_surface() {
    let (mut d, region, regs) = stalled();
    let part = answering_after_reset(&region, &regs, Answer::Spec, u16::MAX);
    settle(&mut d);
    assert_eq!((d.health, d.recoveries), (Health::Live, 1));
    let seen = part.seen();
    let types: Vec<u32> = seen.iter().map(|r| le32(r, 0)).collect();
    let (create, attach) = (VG_CMD_RESOURCE_CREATE_2D, VG_CMD_RESOURCE_ATTACH_BACKING);
    let shown = [VG_CMD_TRANSFER_TO_HOST_2D, VG_CMD_SET_SCANOUT, VG_CMD_RESOURCE_FLUSH];
    assert_eq!(types, [[create, attach].as_slice(), &shown].concat());
    assert_eq!(le32(&seen[0], 24), PRIMARY.resource_id, "made again under the same id");
    assert_eq!(seen[1][32..40], PRIMARY.backing_addr.to_le_bytes(), "on the same frames");
    assert_eq!(le32(&seen[1], 40), PRIMARY.backing_len);
    assert_eq!(call(&d, CLIENT, OP_FLUSH, &whole_flush()), 0, "presenting again");
    assert_eq!(handle(&d), Some(PRIMARY.handle), "under the same surface handle");
    assert_eq!(health(&d), (0, 1));
    assert_eq!(releases(), 0, "the device was never given back");
    let said = said();
    assert!(said[0].contains("(try 1 of 3)") && said[1].contains("recovered in place"));
}

#[test]
fn a_stall_then_a_dead_device_leaves_the_driver_up_answering_e_device() {
    // No part: every try's first command goes unanswered.
    let (mut d, _region, regs) = stalled();
    settle(&mut d);
    assert_eq!(d.health, Health::Down);
    let said = said();
    assert_eq!(said.len(), 4, "three tries and the state, each logged: {said:?}");
    assert!(said[2].contains("(try 3 of 3)") && said[3].contains("down"));
    assert_eq!(regs.wrote8(DEVICE_STATUS), 0, "the device is left reset");
    assert_eq!(call(&d, CLIENT, OP_FLUSH, &whole_flush()), E_DEVICE);
    assert_eq!(handle(&d), Some(PRIMARY.handle), "the surface is still served");
    assert_eq!(health(&d), (1, 0));
    settle(&mut d);
    assert_eq!(nonos_libc::said().len(), 4, "a driver down is not tried again");
    assert_eq!(releases(), 0, "the claim and every grant are kept");
}
