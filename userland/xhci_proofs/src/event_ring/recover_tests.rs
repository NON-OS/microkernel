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

//! Bringing an endpoint back after a failed transfer, and a stuck command
//! aborted, against the producer written from the specification.

use std::sync::Arc;
use std::time::{Duration, Instant};

use nonos_devmodel::{run, FakeBar};

use super::events::{command, CC_SUCCESS};
use super::fixture::{Fixture, INT_DCI, SLOT, TRB};
use crate::constants::CRCR_LO;
use crate::controller::{
    program_command_ring, recover_endpoint, run_command, CC_CONTEXT_STATE_ERROR,
};
use crate::error::XhciError;
use crate::rings::command::CommandRing;
use crate::trb::commands::noop_command;
use crate::trb::Trb;

const RESET_ENDPOINT: u32 = 14;
const STOP_ENDPOINT: u32 = 15;
const SET_TR_DEQUEUE: u32 = 16;

fn ring_base(cmd: &CommandRing) -> u64 {
    cmd.crcr_value() & !0x3F
}

/// The command TRB the driver wrote at bus address `bus`.
fn command_at(bus: u64) -> Trb {
    let host = nonos_libc::dma_host(bus).expect("command ring mapped");
    // SAFETY: a live grant holds the whole TRB.
    unsafe { (host as *const Trb).read() }
}

#[test]
fn a_halted_endpoint_is_reset_and_moved_past_its_ring() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let ring = fx.transfer_ring();
    let base = ring_base(&cmd);
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, &ring, SLOT, INT_DCI)
        .expect("recovered");
    let reset = command_at(base);
    assert_eq!(reset.get_type(), RESET_ENDPOINT);
    assert_eq!(reset.endpoint_id(), INT_DCI);
    assert_eq!(reset.slot_id(), SLOT);
    let set = command_at(base + TRB);
    assert_eq!(set.get_type(), SET_TR_DEQUEUE);
    assert_eq!(set.get_pointer(), ring.enqueue_phys() | ring.cycle() as u64);
}

#[test]
fn an_endpoint_still_running_is_stopped_instead() {
    // A transfer that timed out leaves the endpoint Running: Reset Endpoint
    // answers Context State Error, and Stop Endpoint takes its place.
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let ring = fx.transfer_ring();
    let base = ring_base(&cmd);
    fx.hc.post(command(base, CC_CONTEXT_STATE_ERROR, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + 2 * TRB, CC_SUCCESS, SLOT));
    recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, &ring, SLOT, 1)
        .expect("recovered");
    assert_eq!(command_at(base + TRB).get_type(), STOP_ENDPOINT);
    assert_eq!(command_at(base + 2 * TRB).get_type(), SET_TR_DEQUEUE);
}

#[test]
fn any_other_reset_failure_is_reported_and_the_ring_left_alone() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let ring = fx.transfer_ring();
    let base = ring_base(&cmd);
    fx.hc.post(command(base, 5, SLOT));
    let got = recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, &ring, SLOT, 1);
    assert_eq!(got, Err(XhciError::CommandCompletionFailed(5)));
    assert_eq!(command_at(base + TRB).get_type(), 0, "no further command was queued");
}

const CRCR_CA: u32 = 1 << 2;
const CRCR_CRR: u32 = 1 << 3;

#[test]
fn an_abort_writes_ca_in_the_low_dword_only_and_waits_for_crr() {
    let fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let op = Arc::new(FakeBar::new(0x40));
    program_command_ring(op.base(), &mut cmd);
    op.present32(CRCR_LO as usize + 4, 0xA5A5_A5A5);
    op.present32(CRCR_LO as usize, CRCR_CRR);
    let _hc = run(&op, |bar: &FakeBar| {
        if bar.wrote32(CRCR_LO as usize) & CRCR_CA != 0 {
            bar.present32(CRCR_LO as usize, 0);
        }
    });
    assert!(cmd.abort(), "the ring stopped");
    assert_eq!(op.wrote32(CRCR_LO as usize + 4), 0xA5A5_A5A5, "the high dword is left alone");
}

#[test]
fn a_ring_never_programmed_has_nothing_to_abort() {
    let fx = Fixture::new();
    let cmd = fx.command_ring();
    assert!(!cmd.abort());
}

#[test]
fn a_command_that_never_completes_is_aborted() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let op = Arc::new(FakeBar::new(0x40));
    program_command_ring(op.base(), &mut cmd);
    op.present32(CRCR_LO as usize, CRCR_CRR);
    let _hc = run(&op, |bar: &FakeBar| {
        if bar.wrote32(CRCR_LO as usize) & CRCR_CA != 0 {
            bar.present32(CRCR_LO as usize, 0);
        }
    });
    let started = Instant::now();
    let trb = noop_command(cmd.cycle() != 0);
    let got = run_command(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, trb);
    assert_eq!(got.map(|_| ()), Err(XhciError::CommandCompletionTimeout));
    assert!(started.elapsed() >= Duration::from_secs(5), "Linux's five seconds");
    assert_eq!(op.wrote32(CRCR_LO as usize) & CRCR_CRR, 0, "the abort ran and the ring stopped");
}
