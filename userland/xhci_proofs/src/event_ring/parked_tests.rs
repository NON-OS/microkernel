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

//! The parked table under a flood. It used to give up its oldest event to
//! make room, even the completion an armed keyboard endpoint was waiting
//! for; now only events no armed transfer awaits are given up.

use super::events::{transfer, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, INT_DCI, SLOT, TRB};
use crate::controller::{poll_interrupt_in, IntrPoll};
use crate::error::XhciError;
use crate::rings::event::{IssuedTransfer, ARMED, PARKED};

#[test]
fn an_armed_completion_outlives_a_flood_of_unawaited_events() {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
    assert!(matches!(got, Ok(IntrPoll::Pending)), "armed, nothing came yet");
    let issued = res.interrupt[0].armed.expect("armed");
    // A bulk wait took the report's event off the ring and parked it.
    fx.ring.park(transfer(issued, CC_SUCCESS, 3, SLOT, INT_DCI));
    for i in 0..(PARKED + ARMED) as u64 * 2 {
        fx.ring.park(transfer(0x9000 + i * TRB, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
    }
    assert!(fx.ring.parked_given_up() > 0, "the flood did not fit");
    let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
    assert!(matches!(got, Ok(IntrPoll::Complete(5))), "the report was kept");
}

#[test]
fn a_full_arming_table_refuses_another_arm() {
    let mut fx = Fixture::new();
    for i in 0..ARMED as u64 {
        let issued = IssuedTransfer { phys: 0x9000 + i * TRB, slot: SLOT, dci: BULK_IN_DCI };
        fx.ring.arm(issued).expect("room");
    }
    let one_more = IssuedTransfer { phys: 0x1000, slot: SLOT, dci: INT_DCI };
    assert_eq!(fx.ring.arm(one_more), Err(XhciError::TransferRingFull));
}

#[test]
fn unawaited_events_keep_at_most_the_parked_bound() {
    let mut fx = Fixture::new();
    for i in 0..PARKED as u64 + 3 {
        fx.ring.park(transfer(0x9000 + i * TRB, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
    }
    assert_eq!(fx.ring.parked_given_up(), 3, "the three oldest went");
    let oldest = IssuedTransfer { phys: 0x9000, slot: SLOT, dci: BULK_IN_DCI };
    assert!(fx.ring.take_parked(oldest).is_none());
    let newest = IssuedTransfer { phys: 0x9000 + (PARKED as u64 + 2) * TRB, ..oldest };
    assert!(fx.ring.take_parked(newest).is_some());
}

#[test]
fn a_disabled_slot_leaves_no_arming_behind() {
    let mut fx = Fixture::new();
    for _ in 0..ARMED * 2 {
        let mut res = fx.hid_slot();
        let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
        assert!(matches!(got, Ok(IntrPoll::Pending)), "a replugged keyboard arms again");
        fx.ring.forget_slot(SLOT);
    }
}
