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

//! A second bulk pair beside the first (OP_CONFIGURE_BULK2, 0x23): UAS's
//! data pipes beside its command and status pipes. A reset of it touches
//! its own endpoint and ring (pair_admit_tests.rs has its admission).

use super::command_at::{command_at, ring_base, RESET_ENDPOINT, SET_TR_DEQUEUE};
use super::events::{command, CC_SUCCESS};
use super::fixture::{Fixture, SLOT, TRB};
use super::toggle_slot::{storage_slot, CONTEXT_SIZE};
use crate::controller::{reset_bulk_endpoint, BulkPipes};

const DATA_IN_DCI: u8 = 6;
const DATA_OUT_DCI: u8 = 7;

#[test]
fn a_halted_pipe_of_the_second_pair_is_reset_on_its_own_endpoint() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let mut res = storage_slot(&fx);
    res.bulk2 = Some(BulkPipes::new(&fx.pool, DATA_IN_DCI, DATA_OUT_DCI).expect("pipes"));
    let base = ring_base(&cmd);
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    reset_bulk_endpoint(
        fx.db(),
        fx.intr(),
        &mut cmd,
        &mut fx.ring,
        &res,
        CONTEXT_SIZE,
        (true, false),
    )
    .expect("reset");
    let (reset, dequeue) = (command_at(base), command_at(base + TRB));
    assert_eq!((reset.get_type(), reset.endpoint_id()), (RESET_ENDPOINT, DATA_OUT_DCI));
    assert_eq!((dequeue.get_type(), dequeue.endpoint_id()), (SET_TR_DEQUEUE, DATA_OUT_DCI));
    let ring = &res.bulk2.as_ref().expect("pipes").out_ring;
    assert_eq!(dequeue.get_pointer() & !0xF, ring.enqueue_phys(), "its own ring");
    res.bulk2 = None;
    let none = reset_bulk_endpoint(
        fx.db(),
        fx.intr(),
        &mut cmd,
        &mut fx.ring,
        &res,
        CONTEXT_SIZE,
        (true, true),
    );
    assert!(none.is_err(), "no second pair, nothing to reset");
}
