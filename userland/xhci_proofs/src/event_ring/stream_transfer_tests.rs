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

//! A streamed endpoint's transfer: on its stream's own ring, the stream in
//! the doorbell; and what it refuses (stream_tests.rs has its contexts).

use super::events::{transfer, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, SLOT};
use super::toggle_tests::{storage_slot, CONTEXT_SIZE};
use crate::controller::{
    bulk_transfer, poll_bulk_in, reset_bulk_endpoint, stream_transfer, StreamPipe,
};

#[test]
fn a_transfer_moves_on_its_streams_ring_and_rings_with_its_stream() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    pipes.in_streams = Some(StreamPipe::new(&fx.pool, 2).expect("streams"));
    let at = pipes.in_streams.as_ref().unwrap().rings[1].enqueue_phys();
    fx.hc.post(transfer(at, CC_SUCCESS, 12, SLOT, BULK_IN_DCI));
    let got = stream_transfer((fx.db(), fx.intr()), &mut fx.ring, &mut pipes, (SLOT, true, 2), 512);
    assert_eq!(got, Ok(500));
    assert_eq!(fx.doorbells.wrote32(SLOT as usize * 4), BULK_IN_DCI as u32 | 2 << 16);
    assert!(stream_transfer((fx.db(), fx.intr()), &mut fx.ring, &mut pipes, (SLOT, true, 4), 8)
        .is_err());
    assert!(bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 8).is_err());
    assert!(poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, 8).is_err());
}

#[test]
fn a_streamed_pipe_is_not_reset_as_one_ring() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let mut res = storage_slot(&fx);
    res.bulk.as_mut().unwrap().in_streams = Some(StreamPipe::new(&fx.pool, 1).expect("streams"));
    let reset = reset_bulk_endpoint(
        fx.db(),
        fx.intr(),
        &mut cmd,
        &mut fx.ring,
        &res,
        CONTEXT_SIZE,
        (false, true),
    );
    assert!(reset.is_err(), "its halt is the class driver's to give up on");
}
