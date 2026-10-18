// NONOS Operating System (AGPL-3.0-or-later)
//! Events a hostile device or a confused controller could write, beyond
//! the matching, residual and hand-back proofs beside this: a completion
//! code this driver does not know fails the transfer; a Port Status Change
//! naming port 0 or one past any root port is read and handed back, never
//! acted on; a Transfer Event for a slot never enabled completes nothing.

use super::events::{port_change, transfer, CC_SUCCESS};
use super::fixture::{Fixture, INT_DCI, SLOT, TRB};
use crate::constants::CC_SHORT_PACKET;
use crate::controller::complete;
use crate::controller::drain_events;
use crate::rings::event::IssuedTransfer;

#[test]
fn only_success_and_short_packet_pass_a_transfer() {
    for cc in 0..=u8::MAX {
        let passes = cc == CC_SUCCESS || cc == CC_SHORT_PACKET;
        assert_eq!(complete(cc).is_ok(), passes, "completion code {cc}");
    }
}

#[test]
fn a_port_change_for_no_root_port_is_read_and_left() {
    let mut fx = Fixture::new();
    for port in [0, 255] {
        fx.hc.post(port_change(port));
    }
    let start = fx.dequeue_bus();
    assert_eq!(drain_events(fx.intr(), &mut fx.ring).count, 2);
    assert_eq!(fx.dequeue_bus(), start + 2 * TRB, "consumed, inside the segment");
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn a_transfer_event_for_a_slot_never_enabled_completes_nothing() {
    let mut fx = Fixture::new();
    let issued = IssuedTransfer { phys: 0x1000, slot: SLOT, dci: INT_DCI };
    for (slot, dci) in [(200, INT_DCI), (0, INT_DCI), (SLOT, 31)] {
        fx.ring.park(transfer(0x1000, CC_SUCCESS, 0, slot, dci));
    }
    assert!(fx.ring.take_parked(issued).is_none(), "no stranger's event is its own");
}
