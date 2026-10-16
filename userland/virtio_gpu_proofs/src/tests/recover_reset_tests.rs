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

//! How a recovery try ends. One the device refuses ends in a reset like one
//! it never answers, so once every try has failed the device is left reset
//! and nothing a client sends reaches it. One that works leaves the driver
//! without render context 1: the reset took it with every host resource.

use nonos_libc::releases;
use nonos_virtio::common::DEVICE_STATUS;

use super::client::{answer, call, scanout, transfer};
use super::frames::le32;
use super::part::{answering_after_reset, Answer};
use super::stall::{handle, stalled, whole_flush, CLIENT, PRIMARY};
use crate::clock::now_ms;
use crate::constants::VQ_AVAIL_OFFSET;
use crate::protocol::{E_DEVICE, OP_FLUSH, OP_QUERY_CAPS, OP_SET_SCANOUT, OP_TRANSFER_TO_HOST};
use crate::server::recover::settle;
use crate::state::Health;

#[test]
fn a_try_the_device_refuses_ends_in_a_reset_and_nothing_reaches_it_once_down() {
    let (mut d, region, regs) = stalled();
    let part = answering_after_reset(&region, &regs, Answer::Reject, u16::MAX);
    settle(&mut d);
    assert_eq!(d.health, Health::Down);
    assert!(d.control_queue.is_reset(), "the last refused try left the queue in service");
    assert_eq!(regs.wrote8(DEVICE_STATUS), 0, "the device is left reset");
    assert_eq!(part.seen().len(), 3, "each try reached the device once");
    let (published, from) = (region.wrote16(VQ_AVAIL_OFFSET + 2), now_ms());
    let (id, w, h) = (PRIMARY.resource_id, PRIMARY.width, PRIMARY.height);
    assert_eq!(call(&d, CLIENT, OP_TRANSFER_TO_HOST, &transfer(id, w, h)), E_DEVICE);
    assert_eq!(call(&d, CLIENT, OP_SET_SCANOUT, &scanout(0, id, w, h)), E_DEVICE);
    assert_eq!(call(&d, CLIENT, OP_FLUSH, &whole_flush()), E_DEVICE);
    settle(&mut d);
    assert_eq!(region.wrote16(VQ_AVAIL_OFFSET + 2), published, "published once it was down");
    assert_eq!(now_ms(), from, "a wait on the device once it was down");
    assert_eq!(handle(&d), Some(PRIMARY.handle), "the surface is still served");
    assert_eq!(releases(), 0, "the claim and every grant are kept");
}

#[test]
fn a_recovery_leaves_the_driver_without_its_render_context() {
    let (mut d, region, regs) = stalled();
    d.virgl_ready = true;
    let _part = answering_after_reset(&region, &regs, Answer::Spec, u16::MAX);
    settle(&mut d);
    assert_eq!((d.health, d.recoveries), (Health::Live, 1));
    assert!(!d.virgl_ready, "3D still reported after the reset took context 1");
    let caps = answer(&d, CLIENT, OP_QUERY_CAPS, &[]);
    assert_eq!(le32(&caps, 16), 0, "OP_QUERY_CAPS still offers 3D");
}
