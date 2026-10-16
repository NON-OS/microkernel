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

//! Each transfer consumer of the event ring an edge event can reach, for
//! `boundary_tests` and `boundary_pointer_tests`.

use super::events::{transfer, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, EP0_DCI, INT_DCI, SLOT, TRB};
use crate::controller::{
    bulk_transfer, get_device_descriptor, poll_interrupt_in, IntrPoll, DEVICE_DESCRIPTOR_LEN,
};
use crate::error::XhciResult;
use crate::trb::Trb;

/// What each transfer consumer made of one edge event aimed at its own TRB:
/// the interrupt poll's single answer for an 8-byte request (`None` while
/// pending), then, each with the genuine event posted behind the edge, a
/// 512-byte bulk transfer whose genuine event leaves 100 bytes behind and a
/// device descriptor read whose genuine event succeeds.
#[derive(Debug, PartialEq)]
pub(super) struct Seen {
    /// The report handed up, none, or the error the report ended in.
    pub(super) poll: XhciResult<Option<u16>>,
    pub(super) bulk: XhciResult<u32>,
    pub(super) control: XhciResult<usize>,
}

/// The edge was not taken by any of them.
pub(super) const IGNORED: Seen =
    Seen { poll: Ok(None), bulk: Ok(412), control: Ok(DEVICE_DESCRIPTOR_LEN as usize) };

pub(super) fn through_transfers(edge: impl Fn(u64, u8) -> Trb) -> Seen {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    let issued = res.interrupt[0].ring.enqueue_phys();
    fx.hc.post(edge(issued, INT_DCI));
    let poll = match poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8) {
        Ok(IntrPoll::Complete(n)) => Ok(Some(n)),
        Ok(IntrPoll::Pending) => Ok(None),
        Err(e) => Err(e),
    };
    fx.dequeue_bus();

    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let issued = pipes.in_ring.enqueue_phys();
    fx.hc.post(edge(issued, BULK_IN_DCI));
    fx.hc.post(transfer(issued, CC_SUCCESS, 100, SLOT, BULK_IN_DCI));
    let bulk = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    fx.dequeue_bus();

    let mut fx = Fixture::new();
    let mut ep0 = fx.transfer_ring();
    let status = ep0.enqueue_phys() + 2 * TRB;
    let out = fx.pool.alloc(64).expect("buffer");
    fx.hc.post(edge(status, EP0_DCI));
    fx.hc.post(transfer(status, CC_SUCCESS, 0, SLOT, EP0_DCI));
    let control = get_device_descriptor(fx.db(), fx.intr(), &mut fx.ring, SLOT, &mut ep0, &out);
    fx.dequeue_bus();

    Seen { poll, bulk, control }
}
