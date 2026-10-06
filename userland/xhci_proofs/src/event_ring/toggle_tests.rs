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

//! A bulk pipe the class driver resets comes back with its data toggle at
//! zero on the host side as well, and no event parked for a transfer that
//! was given up is ever taken for a later transfer on the same ring.

use super::events::{command, transfer, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, BULK_OUT_DCI, INT_DCI, SLOT, TRB};
use crate::constants::COMMAND_RING_TRBS;
use crate::controller::{
    bulk_transfer, recover_endpoint, reset_bulk_endpoint, run_command, write_bulk_input,
    BulkEndpoint, Recovered, CC_CONTEXT_STATE_ERROR,
};
use crate::dma::DmaRegion;
use crate::rings::command::CommandRing;
use crate::rings::event::IssuedTransfer;
use crate::slots::SlotResources;
use crate::trb::commands::noop_command;
use crate::trb::Trb;

const CONTEXT_SIZE: u8 = 32;
const RESET_ENDPOINT: u32 = 14;
const STOP_ENDPOINT: u32 = 15;
const SET_TR_DEQUEUE: u32 = 16;
const CONFIGURE_ENDPOINT: u32 = 12;
const CC_STOPPED: u8 = 26;
const CC_STOPPED_LENGTH_INVALID: u8 = 27;
const CC_STOPPED_SHORT_PACKET: u8 = 28;
const EP_STATE_STOPPED: u32 = 3;

fn ring_base(cmd: &CommandRing) -> u64 {
    cmd.crcr_value() & !0x3F
}

fn command_at(bus: u64) -> Trb {
    let host = nonos_libc::dma_host(bus).expect("command ring mapped");
    // SAFETY: a live grant holds the whole TRB.
    unsafe { (host as *const Trb).read() }
}

fn dw(region: &DmaRegion, context: usize, dword: usize) -> u32 {
    let at = context * CONTEXT_SIZE as usize + dword * 4;
    // SAFETY: every context region holds 33 contexts of CONTEXT_SIZE bytes.
    unsafe { (region.as_mut_ptr::<u8>().add(at) as *const u32).read_volatile() }
}

fn set_dw(region: &DmaRegion, context: usize, dword: usize, value: u32) {
    let at = context * CONTEXT_SIZE as usize + dword * 4;
    // SAFETY: as for `dw`.
    unsafe { (region.as_mut_ptr::<u8>().add(at) as *mut u32).write_volatile(value) }
}

/// An addressed SuperSpeed slot with bulk pipes, its output context holding
/// what a controller holds after Configure Endpoint: Context Entries 5 and
/// both bulk endpoints Stopped, with a burst of 15 and a 1024-byte packet.
fn storage_slot(fx: &Fixture) -> SlotResources {
    let mut res =
        SlotResources::allocate(&fx.pool, CONTEXT_SIZE, SLOT, 1, 4, true).expect("slot resources");
    res.bulk = Some(fx.bulk_pipes());
    set_dw(&res.output_context, 0, 0, 5 << 27 | 4 << 20);
    for dci in [BULK_IN_DCI, BULK_OUT_DCI] {
        let ep = dci as usize;
        set_dw(&res.output_context, ep, 0, EP_STATE_STOPPED);
        set_dw(&res.output_context, ep, 1, 3 << 1 | 6 << 3 | 15 << 8 | 1024 << 16);
        set_dw(&res.output_context, ep, 2, 0xDEAD_0001);
        set_dw(&res.output_context, ep, 3, 0);
        set_dw(&res.output_context, ep, 4, 3 * 1024);
    }
    res
}

#[test]
fn a_halted_bulk_pipe_is_reset_and_nothing_more() {
    // Reset Endpoint with TSP clear already zeroes the toggle.
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let res = storage_slot(&fx);
    let base = ring_base(&cmd);
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    reset_bulk_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, &res, CONTEXT_SIZE, true)
        .expect("reset");
    assert_eq!(command_at(base).get_type(), RESET_ENDPOINT);
    assert_eq!(command_at(base + TRB).get_type(), SET_TR_DEQUEUE);
    assert_eq!(command_at(base + 2 * TRB).get_type(), 0, "no Configure Endpoint");
}

#[test]
fn a_bulk_pipe_that_had_not_halted_is_dropped_and_added_again() {
    // The case that kept a USB stick out of step: the pipe timed out, was
    // only stopped, and CLEAR_FEATURE set the device's toggle back to DATA0.
    for (dir_in, dci) in [(true, BULK_IN_DCI), (false, BULK_OUT_DCI)] {
        let mut fx = Fixture::new();
        let mut cmd = fx.command_ring();
        let mut res = storage_slot(&fx);
        // A few transfers already went through, so the ring is mid-way.
        {
            let pipes = res.bulk.as_mut().expect("pipes");
            let ring = if dir_in { &mut pipes.in_ring } else { &mut pipes.out_ring };
            for _ in 0..5 {
                ring.enqueue(Trb::zero()).expect("room");
            }
        }
        let base = ring_base(&cmd);
        fx.hc.post(command(base, CC_CONTEXT_STATE_ERROR, SLOT));
        fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
        fx.hc.post(command(base + 2 * TRB, CC_SUCCESS, SLOT));
        fx.hc.post(command(base + 3 * TRB, CC_SUCCESS, SLOT));
        reset_bulk_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, &res, CONTEXT_SIZE, dir_in)
            .expect("reset");
        assert_eq!(command_at(base + TRB).get_type(), STOP_ENDPOINT);
        assert_eq!(command_at(base + 2 * TRB).get_type(), SET_TR_DEQUEUE);
        let configure = command_at(base + 3 * TRB);
        assert_eq!(configure.get_type(), CONFIGURE_ENDPOINT);
        assert_eq!(configure.slot_id(), SLOT);
        assert_eq!(configure.get_pointer(), res.input_context.phys());

        let input = &res.input_context;
        assert_eq!(dw(input, 0, 0), 1 << dci, "dropped");
        assert_eq!(dw(input, 0, 1), 1 | 1 << dci, "slot and endpoint added");
        assert!(dw(input, 1, 0) >> 27 >= dci as u32, "Context Entries covers it");
        let ep = dci as usize + 1;
        assert_eq!(dw(input, ep, 0) & 7, 0, "the state is not handed back");
        assert_eq!(
            dw(input, ep, 1),
            3 << 1 | 6 << 3 | 15 << 8 | 1024 << 16,
            "type, burst, mps kept"
        );
        let pipes = res.bulk.as_ref().expect("pipes");
        let ring = if dir_in { &pipes.in_ring } else { &pipes.out_ring };
        let deq = dw(input, ep, 2) as u64 | (dw(input, ep, 3) as u64) << 32;
        assert_eq!(deq, ring.enqueue_phys() | ring.cycle() as u64, "the ring goes on from here");
        assert_eq!(dw(input, ep, 4), 3 * 1024);
        let other = if dir_in { BULK_OUT_DCI } else { BULK_IN_DCI } as usize + 1;
        assert_eq!(dw(input, other, 1), 0, "the other pipe is left alone");
    }
}

#[test]
fn recovery_says_which_way_the_endpoint_came_back() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let ring = fx.transfer_ring();
    let base = ring_base(&cmd);
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    let got = recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, &ring, SLOT, 1);
    assert_eq!(got, Ok(Recovered::Reset));
    fx.hc.post(command(base + 2 * TRB, CC_CONTEXT_STATE_ERROR, SLOT));
    fx.hc.post(command(base + 3 * TRB, CC_CONTEXT_STATE_ERROR, SLOT));
    fx.hc.post(command(base + 4 * TRB, CC_SUCCESS, SLOT));
    let got = recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, &ring, SLOT, 1);
    assert_eq!(got, Ok(Recovered::Stopped), "already Stopped is not Halted either");
}

#[test]
fn a_stopped_event_is_never_parked() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let base = ring_base(&cmd);
    let pipes = fx.bulk_pipes();
    let at = pipes.in_ring.enqueue_phys();
    for (i, code) in [CC_STOPPED, CC_STOPPED_LENGTH_INVALID, CC_STOPPED_SHORT_PACKET, CC_SUCCESS]
        .into_iter()
        .enumerate()
    {
        fx.hc.post(transfer(at, code, 0, SLOT, BULK_IN_DCI));
        fx.hc.post(command(base + i as u64 * TRB, CC_SUCCESS, 0));
        let noop = noop_command(cmd.cycle() != 0);
        run_command(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, noop).expect("noop");
        let issued = IssuedTransfer { phys: at, slot: SLOT, dci: BULK_IN_DCI };
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
    // A late completion of the bulk TRB that timed out, and an interrupt
    // report, both arrive while the recovery's commands run.
    fx.hc.post(transfer(bulk_at, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
    fx.hc.post(transfer(int_at, CC_SUCCESS, 0, SLOT, INT_DCI));
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, &pipes.in_ring, SLOT, BULK_IN_DCI)
        .expect("recovered");
    let bulk = IssuedTransfer { phys: bulk_at, slot: SLOT, dci: BULK_IN_DCI };
    let int = IssuedTransfer { phys: int_at, slot: SLOT, dci: INT_DCI };
    assert!(fx.ring.take_parked(bulk).is_none(), "the given-up transfer's event is gone");
    assert!(fx.ring.take_parked(int).is_some(), "another endpoint's event is kept");
}

#[test]
fn a_transfer_one_lap_later_is_not_answered_by_a_stale_event() {
    // Before the fix: the late completion of a transfer that timed out sat
    // parked, and the TRB written at the same address one lap round the
    // 64-TRB ring was taken as complete at once, with the old residual.
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let mut pipes = fx.bulk_pipes();
    let base = ring_base(&cmd);
    let first = pipes.in_ring.enqueue_phys();
    fx.hc.post(transfer(first, CC_SUCCESS, 13, SLOT, BULK_IN_DCI));
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, &pipes.in_ring, SLOT, BULK_IN_DCI)
        .expect("recovered");
    // Every other TRB of the lap completes normally.
    for _ in 0..COMMAND_RING_TRBS - 1 {
        let at = pipes.in_ring.enqueue_phys();
        if at == first {
            break;
        }
        fx.hc.post(transfer(at, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
        let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
        assert_eq!(moved, Ok(512));
    }
    assert_eq!(pipes.in_ring.enqueue_phys(), first, "one lap round");
    fx.hc.post(transfer(first, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
    let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    assert_eq!(moved, Ok(512), "its own event, not the stale residual of 13");
}

#[test]
fn a_superspeed_burst_is_written_into_the_endpoint_context() {
    let fx = Fixture::new();
    let res = storage_slot(&fx);
    let ep =
        |dci, burst| BulkEndpoint { dci, ring_phys: 0x10_0000, max_packet: 1024, max_burst: burst };
    write_bulk_input(
        &res.input_context,
        &res.output_context,
        CONTEXT_SIZE,
        ep(BULK_IN_DCI, 15),
        ep(BULK_OUT_DCI, 3),
    );
    let burst = |dci: u8| (dw(&res.input_context, dci as usize + 1, 1) >> 8) & 0xFF;
    assert_eq!(burst(BULK_IN_DCI), 15);
    assert_eq!(burst(BULK_OUT_DCI), 3);
    write_bulk_input(
        &res.input_context,
        &res.output_context,
        CONTEXT_SIZE,
        ep(BULK_IN_DCI, 0x1F),
        ep(BULK_OUT_DCI, 0),
    );
    assert_eq!(burst(BULK_IN_DCI), 15, "bMaxBurst is four bits");
    assert_eq!(burst(BULK_OUT_DCI), 0);
}
