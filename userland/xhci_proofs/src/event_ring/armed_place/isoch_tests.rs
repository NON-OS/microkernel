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

use super::super::events::{transfer, CC_SUCCESS};
use super::super::fixture::{Fixture, BULK_OUT_DCI, SLOT, TRB};
use super::super::isoch_pipe::{pipe, send, CC_MISSED, DCI};
use super::take_every_place;
use crate::controller::bulk_transfer;
use crate::error::XhciError;
use crate::protocol::ISOCH_PACKETS_MAX;

/// The isoch ring's slots before its Link: two whole batches and one.
const ISOCH_SLOTS: u64 = 2 * ISOCH_PACKETS_MAX as u64 + 1;

#[test]
fn with_every_place_taken_no_batch_is_queued() {
    let mut fx = Fixture::new();
    let mut p = pipe(&fx);
    take_every_place(&mut fx);
    assert_eq!(send(&mut fx, &mut p, &[192], 1), Err(XhciError::TooManyArmed));
    assert_eq!(p.ring.enqueue_phys(), p.ring.phys(), "no TD queued");
}

#[test]
fn a_batchs_completion_read_by_a_bulk_wait_is_kept_for_the_next_batch() {
    let mut fx = Fixture::new();
    let (mut pipes, mut p) = (fx.bulk_pipes(), pipe(&fx));
    send(&mut fx, &mut p, &[192], 1).unwrap();
    fx.hc.post(transfer(p.ring.phys(), CC_MISSED, 0, SLOT, DCI));
    fx.hc.post(transfer(pipes.out_ring.enqueue_phys(), CC_SUCCESS, 0, SLOT, BULK_OUT_DCI));
    let sent = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, false, 60);
    assert_eq!((sent, send(&mut fx, &mut p, &[192], 1)), (Ok(60), Ok(CC_MISSED)));
}

#[test]
fn full_batches_keep_coming_and_a_ring_left_full_refuses_the_next_whole() {
    let mut fx = Fixture::new();
    let mut p = pipe(&fx);
    let (lens, base, n) = ([6u16; ISOCH_PACKETS_MAX], p.ring.phys(), ISOCH_PACKETS_MAX as u64);
    let last = |k: u64| base + ((n * k + n - 1) % ISOCH_SLOTS) * TRB;
    assert_eq!(send(&mut fx, &mut p, &lens, 0), Ok(CC_SUCCESS));
    for k in 1..8u64 {
        fx.hc.post(transfer(last(k - 1), CC_SUCCESS, 0, SLOT, DCI));
        assert_eq!(send(&mut fx, &mut p, &lens, k as u8), Ok(CC_SUCCESS), "batch {k}");
    }
    // Nothing completes: the wait gives up, and both batches stay queued.
    assert_eq!(send(&mut fx, &mut p, &lens, 8), Err(XhciError::TransferCompletionTimeout));
    let at = p.ring.enqueue_phys();
    assert_eq!(send(&mut fx, &mut p, &lens, 9), Err(XhciError::TransferRingFull));
    assert_eq!(p.ring.enqueue_phys(), at, "no part of the batch queued");
}
