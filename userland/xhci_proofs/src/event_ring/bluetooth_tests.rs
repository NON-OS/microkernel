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

//! A Bluetooth controller's slot: HCI events on an interrupt-IN endpoint
//! beside ACL data on a bulk pair, one interface. Each configure takes its
//! own endpoints whatever the other holds (bluetooth_limit_tests.rs has the
//! read's size, bluetooth_poll_tests.rs the two polled side by side).

use super::fixture::{Fixture, BULK_OUT_DCI, INT_DCI, SLOT};
use crate::contexts::with_entries;
use crate::slots::{interrupt_limit, InterruptEndpoint, SlotResources};

/// HCI event packets of the controller's endpoint, a common size.
pub(super) const EVENT_PACKET: u16 = 16;
const CONTEXT_SIZE: u8 = 32;

/// A slot holding the interrupt endpoint and the bulk pair, as the two
/// configure handlers leave it in either order.
pub(super) fn radio_slot(fx: &Fixture) -> SlotResources {
    let mut res =
        SlotResources::allocate(&fx.pool, CONTEXT_SIZE, SLOT, 1, 3, false).expect("slot resources");
    assert!(res.takes_bulk() && res.takes_interrupt());
    res.bulk = Some(fx.bulk_pipes());
    assert!(res.takes_interrupt(), "bulk pipes leave room for the event endpoint");
    let max = interrupt_limit(EVENT_PACKET);
    let buf = fx.pool.alloc(max as u64).expect("event buffer");
    res.interrupt.push(InterruptEndpoint {
        dci: INT_DCI,
        ring: fx.transfer_ring(),
        buf,
        max,
        armed: None,
    });
    assert!(!res.takes_bulk(), "one bulk pair a slot");
    res
}

#[test]
fn a_slot_holds_an_interrupt_endpoint_and_a_bulk_pair() {
    let fx = Fixture::new();
    let mut res = radio_slot(&fx);
    while res.takes_interrupt() {
        let buf = fx.pool.alloc(8).expect("buffer");
        let ep = InterruptEndpoint { dci: 7, ring: fx.transfer_ring(), buf, max: 8, armed: None };
        res.interrupt.push(ep);
    }
    assert_eq!(res.interrupt.len(), 4, "the bulk pair does not count against the four");
    assert!(res.bulk.is_some() && !res.takes_bulk());
}

#[test]
fn the_second_configure_keeps_the_first_ones_endpoints() {
    let entries = |dw0: u32| dw0 >> 27;
    // Bulk first (OUT at DCI 5), then the event endpoint at DCI 3.
    assert_eq!(entries(with_entries(with_entries(0, BULK_OUT_DCI), INT_DCI)), 5);
    // The event endpoint first, then the bulk pair.
    assert_eq!(entries(with_entries(with_entries(0, INT_DCI), BULK_OUT_DCI)), 5);
    let others = 0x07FF_FFFF;
    assert_eq!(with_entries(others, INT_DCI) & others, others, "nothing else changes");
}
