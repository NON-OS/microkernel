// NONOS Operating System (AGPL-3.0-or-later)
//! A control read that a device answers short: the data stage asks for a
//! Short Packet event, the residual on it is read, and the bytes reported
//! are the bytes sent, never the bytes asked.

use crate::constants::{CC_SHORT_PACKET, CC_SUCCESS, TRB_ISP, TRB_TYPE_TRANSFER_EVENT};

/// A STALL (xHCI 6.4.5), as recalled, unverified.
const CC_STALL: u8 = 6;
use crate::controller::transfer::issue_control_transfer::moved;
use crate::controller::transfer::transfer_event::{short_on, stopped_early};
use crate::rings::event::IssuedTransfer;
use crate::trb::builders::data_stage::data_stage_in;
use crate::trb::Trb;

const SETUP: u64 = 0x1000;
const DATA: u64 = 0x1010;
const STATUS: u64 = 0x1020;
const ISSUED: IssuedTransfer = IssuedTransfer { phys: STATUS, slot: 3, dci: 1 };

fn event(on: u64, cc: u8, residual: u32, slot: u8) -> Trb {
    let mut t = Trb::zero();
    t.set_pointer(on);
    t.d2 = u32::from(cc) << 24 | residual;
    t.d3 = u32::from(slot) << 24 | 1 << 16;
    t.set_type(TRB_TYPE_TRANSFER_EVENT);
    t
}

#[test]
fn the_data_stage_asks_to_hear_of_a_short_answer() {
    assert_ne!(data_stage_in(DATA, 512, true).d3 & TRB_ISP, 0);
}

#[test]
fn a_short_data_stage_reports_what_it_did_not_fill() {
    let short = event(DATA, CC_SHORT_PACKET, 503, 3);
    assert_eq!(short_on(&short, ISSUED, &[SETUP, DATA]), Some(503));
    assert!(!stopped_early(&short, ISSUED, &[SETUP, DATA]), "short is no error");
    assert_eq!(moved(512, 503), 9);
}

#[test]
fn only_this_transfers_short_event_counts() {
    let earlier = [SETUP, DATA];
    assert_eq!(short_on(&event(0x9000, CC_SHORT_PACKET, 5, 3), ISSUED, &earlier), None);
    assert_eq!(short_on(&event(DATA, CC_SHORT_PACKET, 5, 4), ISSUED, &earlier), None);
    assert_eq!(short_on(&event(DATA, CC_SUCCESS, 5, 3), ISSUED, &earlier), None);
    assert!(stopped_early(&event(DATA, CC_STALL, 0, 3), ISSUED, &earlier));
}

#[test]
fn a_residual_past_the_request_reads_as_nothing() {
    assert_eq!(moved(18, 0), 18);
    assert_eq!(moved(18, 18), 0);
    assert_eq!(moved(18, 0x00FF_FFFF), 0);
    let wild = event(DATA, CC_SHORT_PACKET, 0x00FF_FFFF, 3);
    assert_eq!(short_on(&wild, ISSUED, &[SETUP, DATA]).map(|r| moved(64, r)), Some(0));
}
