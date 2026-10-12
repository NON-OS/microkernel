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

//! A driver as bring-up leaves it, its primary surface handed to the
//! client, which presents a frame; the device then hangs on the next.

use std::sync::Arc;

use nonos_devmodel::FakeBar;

use super::client::{answer, call, flush};
use super::frames::le32;
use super::harness::{driver, harness};
use super::part::{answering, Answer};
use crate::driver::Driver;
use crate::protocol::{E_DEVICE, OP_FLUSH, OP_GET_PRIMARY_SURFACE, OP_HEALTHCHECK};
use crate::setup::{resource, Primary};
use crate::state::Scanout;

pub const CLIENT: u32 = 5;
pub const PRIMARY: Primary = Primary {
    handle: 0x77,
    resource_id: 1,
    width: 64,
    height: 32,
    stride: 256,
    backing_addr: 0x8000_0000,
    backing_len: 64 * 32 * 4,
};

/// The driver once its queue stalled, with the ring region and the window.
pub fn stalled() -> (Driver, Arc<FakeBar>, Arc<FakeBar>) {
    let (d, region, regs) = driver(harness(), Some(PRIMARY));
    let (width, height) = (PRIMARY.width, PRIMARY.height);
    let whole = Scanout { x: 0, y: 0, width, height, current_resource_id: 0, enabled: true };
    resource::insert(&d.resources, PRIMARY.resource_id, whole).expect("room");
    assert_eq!(handle(&d), Some(PRIMARY.handle), "the client holds the surface");
    let part = answering(&region, Answer::Spec);
    assert_eq!(call(&d, CLIENT, OP_FLUSH, &whole_flush()), 0, "a frame presented");
    drop(part);
    assert_eq!(call(&d, CLIENT, OP_FLUSH, &whole_flush()), E_DEVICE);
    assert!(d.control_queue.is_reset(), "the flush stalled");
    (d, region, regs)
}

/// A flush of the whole primary surface.
pub fn whole_flush() -> Vec<u8> {
    flush(PRIMARY.resource_id, 0, 0, PRIMARY.width, PRIMARY.height)
}

/// The surface handle OP_GET_PRIMARY_SURFACE gives the client.
pub fn handle(d: &Driver) -> Option<u64> {
    let reply = answer(d, CLIENT, OP_GET_PRIMARY_SURFACE, &[]);
    (le32(&reply, 0) == 0).then(|| u64::from_le_bytes(reply[4..12].try_into().expect("8")))
}

/// OP_HEALTHCHECK's state word and count of stalls recovered.
pub fn health(d: &Driver) -> (u32, u32) {
    let reply = answer(d, CLIENT, OP_HEALTHCHECK, &[]);
    assert_eq!(le32(&reply, 0), 0, "the driver answers in every state");
    (le32(&reply, 4), le32(&reply, 8))
}
