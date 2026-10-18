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

//! No event parked for a transfer that was given up is ever taken for a
//! later transfer on the same ring. Only an armed TRB's event is parked,
//! so each test arms the TRBs whose events it posts (`expect`).

use super::command_at::{endpoint, quiet_context, ring_base};
use super::events::{command, transfer, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, INT_DCI, SLOT, TRB};
use crate::controller::command::run_command::run_command;
use crate::controller::recover_endpoint;
use crate::rings::event::IssuedTransfer;
use crate::trb::commands::noop_command;

const CC_STOPPED: u8 = 26;
const CC_STOPPED_LENGTH_INVALID: u8 = 27;
const CC_STOPPED_SHORT_PACKET: u8 = 28;

#[test]
fn a_stopped_event_is_never_parked() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let base = ring_base(&cmd);
    let pipes = fx.bulk_pipes();
    let at = pipes.in_ring.enqueue_phys();
    let codes = [CC_STOPPED, CC_STOPPED_LENGTH_INVALID, CC_STOPPED_SHORT_PACKET, CC_SUCCESS];
    for (i, code) in codes.into_iter().enumerate() {
        let issued = IssuedTransfer { phys: at, slot: SLOT, dci: BULK_IN_DCI };
        assert!(fx.ring.expect(issued));
        fx.hc.post(transfer(at, code, 0, SLOT, BULK_IN_DCI));
        fx.hc.post(command(base + i as u64 * TRB, CC_SUCCESS, 0));
        let noop = noop_command(cmd.cycle() != 0);
        run_command(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, noop).expect("noop");
        assert_eq!(fx.ring.take_parked(issued).is_some(), code == CC_SUCCESS, "code {code}");
    }
}

#[test]
fn recovering_an_endpoint_drops_its_parked_events_and_no_others() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let pipes = fx.bulk_pipes();
    let base = ring_base(&cmd);
    let bulk_at = pipes.in_ring.enqueue_phys();
    let int_at = 0x1234_5670;
    let bulk = IssuedTransfer { phys: bulk_at, slot: SLOT, dci: BULK_IN_DCI };
    let int = IssuedTransfer { phys: int_at, slot: SLOT, dci: INT_DCI };
    assert!(fx.ring.expect(bulk) && fx.ring.expect(int));
    // A late completion of the bulk TRB that timed out, and an interrupt
    // report, both arrive while the recovery's commands run.
    fx.hc.post(transfer(bulk_at, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
    fx.hc.post(transfer(int_at, CC_SUCCESS, 0, SLOT, INT_DCI));
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    let out = quiet_context(&fx);
    let ep = endpoint(&pipes.in_ring, &out, BULK_IN_DCI);
    recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, ep).expect("recovered");
    assert!(fx.ring.take_parked(bulk).is_none(), "the given-up transfer's event is gone");
    assert!(fx.ring.take_parked(int).is_some(), "another endpoint's event is kept");
}
