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

//! The wait an isoch batch makes for the one before: its own TD's event,
//! whatever came ahead of it; a late batch reported, a failed one an error.

use super::events::transfer;
use super::fixture::{Fixture, BULK_IN_DCI, SLOT, TRB};
use super::isoch_pipe::DCI;
use super::isoch_pipe::{pipe, send, trb_at, CC_MISSED, CC_SUCCESS, CC_TRANSACTION, CC_UNDERRUN};
use crate::error::XhciError;
use crate::protocol::ISOCH_MAX;
use crate::rings::event::IssuedTransfer;

#[test]
fn the_next_batch_takes_the_other_half_and_the_last_batchs_completion() {
    let mut fx = Fixture::new();
    let mut p = pipe(&fx);
    send(&mut fx, &mut p, &[192, 192], 1).unwrap();
    let last = p.ring.phys() + TRB;
    // A bulk IN armed elsewhere, with a place for its event (`expect`).
    let other = IssuedTransfer { phys: 0x9000, slot: SLOT, dci: BULK_IN_DCI };
    assert!(fx.ring.expect(other));
    fx.hc.post(transfer(0, CC_UNDERRUN, 0, SLOT, DCI));
    fx.hc.post(transfer(0, CC_MISSED, 0, SLOT, DCI));
    fx.hc.post(transfer(0x9000, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
    fx.hc.post(transfer(last, CC_SUCCESS, 0, SLOT, DCI));
    assert_eq!(send(&mut fx, &mut p, &[192], 2), Ok(CC_SUCCESS));
    assert_eq!(trb_at(p.ring.phys() + 2 * TRB).get_pointer(), p.buf.phys() + ISOCH_MAX as u64);
    assert!(fx.ring.take_parked(other).is_some(), "another endpoint's event is kept");
    let dry = IssuedTransfer { phys: 0, slot: SLOT, dci: DCI };
    assert!(fx.ring.take_parked(dry).is_none(), "its own endpoint's strays are not");
    fx.ring.park(transfer(0, CC_UNDERRUN, 0, SLOT, DCI));
    assert!(fx.ring.take_parked(dry).is_none(), "an underrun is never parked");
}

#[test]
fn a_late_batch_is_reported_and_a_failed_one_is_an_error() {
    let mut fx = Fixture::new();
    let mut p = pipe(&fx);
    send(&mut fx, &mut p, &[192], 1).unwrap();
    fx.hc.post(transfer(p.ring.phys(), CC_MISSED, 0, SLOT, DCI));
    assert_eq!(send(&mut fx, &mut p, &[192], 1), Ok(CC_MISSED));
    fx.hc.post(transfer(p.ring.phys() + TRB, CC_TRANSACTION, 0, SLOT, DCI));
    let failed = send(&mut fx, &mut p, &[192], 1);
    assert_eq!(failed, Err(XhciError::TransferCompletionFailed(CC_TRANSACTION)));
}
