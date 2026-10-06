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

//! A controller that writes anything at all into the event ring.
//!
//! Every round posts up to four events whose every dword is random, with the
//! fields that steer the driver drawn half the time from the values that
//! matter: the type from transfer, command, port change, 0 and 63; the
//! pointer from the issued TRB itself, the same TRB with reserved bits set,
//! its neighbours, the high half flipped, null, below 16 and all ones; the
//! slot from the configured one, one never enabled, 0 and 255; the endpoint
//! from the configured one, endpoint 1, 0 and 31; the third dword from all
//! ones and residuals at and past the request. One of the driver's
//! consumers then takes the ring: the drain,
//! the interrupt-IN poll, a bulk transfer, a control transfer or an Enable
//! Slot command. The waits get their genuine event posted behind the noise,
//! so each round ends. Some rounds leave a half-written TRB, its cycle bit
//! the wrong way, at the enqueue slot.
//!
//! After every round nothing has panicked, the dequeue pointer and ERDP lie
//! in the segment on a TRB boundary with DESI zero, ERDP names the next
//! unread TRB, no TRB from past the segment was read, every length handed up
//! is within its request and its buffer, every slot id taken from an answer
//! is refused unless the slot table can hold it, and the producer never had
//! to write over an event the driver had not handed back.

use super::events::{
    command, transfer, CC_SHORT_PACKET, CC_STALL, CC_SUCCESS, CC_TRB_ERROR, PORT_STATUS_CHANGE,
};
use super::fixture::{Fixture, BULK_IN_DCI, EP0_DCI, INT_DCI, SLOT, TRB};
use super::producer::{is_poison, ERDP_EHB_BIT, ERDP_FLAGS};
use crate::constants::{
    COMMAND_RING_TRBS, TRB_TYPE_CMD_COMPLETION_EVENT, TRB_TYPE_TRANSFER_EVENT,
};
use crate::controller::wait_command_completion::wait_command_completion;
use crate::controller::{
    bulk_transfer, drain_events, get_device_descriptor, issue_enable_slot, poll_interrupt_in,
    BulkPipes, IntrPoll, DEVICE_DESCRIPTOR_LEN, DRAIN_BATCH,
};
use crate::dma::DmaRegion;
use crate::error::XhciError;
use crate::protocol::{BULK_MAX, HID_REPORT_MAX};
use crate::rings::command::CommandRing;
use crate::rings::transfer::TransferRing;
use crate::slots::{SlotResources, SlotTable};
use crate::trb::commands::noop_command;
use crate::trb::Trb;

const ROUNDS: u32 = 200_000;
const MAX_SLOTS: u8 = 8;
/// Room kept free so a round's noise and its genuine event always fit.
const ROUND_ROOM: usize = 6;
/// Usable TRBs on a command or transfer ring; the last holds the link.
const RING_TRBS: u64 = COMMAND_RING_TRBS as u64 - 1;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    fn coin(&mut self) -> bool {
        self.next() & 1 == 0
    }
    fn pick<T: Copy>(&mut self, from: &[T]) -> T {
        from[self.below(from.len() as u64) as usize]
    }
}

/// An event with every dword random, its steering fields half the time from
/// the edges named above, aimed at the TRB at `issued` on endpoint `dci`.
fn hostile(rng: &mut Rng, issued: u64, dci: u8, request: u32) -> Trb {
    let mut t = random_trb(rng);
    if rng.coin() {
        let ty = rng.pick(&[
            TRB_TYPE_TRANSFER_EVENT,
            TRB_TYPE_CMD_COMPLETION_EVENT,
            PORT_STATUS_CHANGE,
            0,
            63,
        ]);
        t.set_type(ty);
    }
    if rng.coin() {
        let low = rng.below(16);
        let p = rng.pick(&[
            issued,
            issued | low,
            issued + TRB,
            issued.wrapping_sub(TRB),
            issued ^ (1 << 40),
            0,
            low,
            u64::MAX,
        ]);
        t.set_pointer(p);
    }
    if rng.coin() {
        let slot = rng.pick(&[SLOT, 0, 255, SLOT + 1]) as u32;
        t.d3 = (t.d3 & 0x00FF_FFFF) | slot << 24;
    }
    if rng.coin() {
        let ep = rng.pick(&[dci, 0, 31, EP0_DCI]) as u32;
        t.d3 = (t.d3 & !(0x1F << 16)) | ep << 16;
    }
    if rng.coin() {
        t.d2 = third_dword(rng, request);
    }
    t
}

fn random_trb(rng: &mut Rng) -> Trb {
    let r = rng.next();
    Trb { d0: r as u32, d1: (r >> 32) as u32, d2: rng.next() as u32, d3: rng.next() as u32 }
}

/// A completion code and residual for a `request`-byte TRB, from the edges.
fn third_dword(rng: &mut Rng, request: u32) -> u32 {
    let code = rng.pick(&[CC_SUCCESS, CC_SHORT_PACKET, CC_STALL, CC_TRB_ERROR, 0, 0xFF]) as u32;
    let residual = rng.pick(&[0, 1, request.saturating_sub(1), request, request + 1, 0x00FF_FFFF]);
    let near = rng.below(request as u64 + 2) as u32;
    let any = rng.next() as u32;
    rng.pick(&[u32::MAX, code << 24 | residual, code << 24 | near, any])
}

/// The event that names `issued` on `dci`, as a working controller would
/// write it, with a completion code and residual from the edges.
fn genuine(rng: &mut Rng, issued: u64, dci: u8, request: u32) -> Trb {
    let mut t = transfer(issued, CC_SUCCESS, 0, SLOT, dci);
    if rng.coin() {
        t.d2 = third_dword(rng, request);
    }
    t
}

/// One driver, its rings and endpoints, kept across every round so the
/// rings go round many times and the parked events carry over.
struct Driver {
    fx: Fixture,
    cmd: CommandRing,
    cmd_issued: u64,
    flush: CommandRing,
    ep0: TransferRing,
    desc: DmaRegion,
    hid: SlotResources,
    pipes: BulkPipes,
    table: SlotTable,
}

impl Driver {
    fn new() -> Self {
        let fx = Fixture::new();
        let cmd = fx.command_ring();
        let flush = fx.command_ring();
        let ep0 = fx.transfer_ring();
        let desc = fx.pool.alloc(64).expect("descriptor buffer");
        let hid = fx.hid_slot();
        let pipes = fx.bulk_pipes();
        Self { cmd, cmd_issued: 0, flush, ep0, desc, hid, pipes, table: SlotTable::new(), fx }
    }

    /// Where the next command on `cmd` goes: its ring wraps before the link.
    fn next_command(&self) -> u64 {
        (self.cmd.crcr_value() & !0x3F) + (self.cmd_issued % RING_TRBS) * TRB
    }

    /// Hand back everything pending through a wait on a ring no noise names.
    fn flush(&mut self) {
        let issued = self.flush.enqueue(noop_command(self.flush.cycle() != 0)).expect("noop");
        self.fx.hc.post(command(issued, CC_SUCCESS, 1));
        let done = wait_command_completion(self.fx.intr(), issued, &mut self.fx.ring);
        assert_eq!(done.map(|c| c.slot_id), Ok(1));
        assert_eq!(self.fx.hc.pending(), 0);
    }

    fn noise(&mut self, rng: &mut Rng, issued: u64, dci: u8, request: u32) {
        for _ in 0..rng.below(5) {
            let event = hostile(rng, issued, dci, request);
            self.fx.hc.post(event);
        }
    }

    fn round(&mut self, rng: &mut Rng) {
        if self.fx.hc.room() < ROUND_ROOM {
            self.flush();
        }
        match rng.below(5) {
            0 => self.drain(rng),
            1 => self.poll(rng),
            2 => self.bulk(rng),
            3 => self.control(rng),
            _ => self.enable_slot(rng),
        }
        if rng.below(8) == 0 {
            let half_written = random_trb(rng);
            self.fx.hc.scribble(half_written);
        }
        self.check();
    }

    fn drain(&mut self, rng: &mut Rng) {
        let issued = self.hid.interrupt[0].armed.unwrap_or(0);
        self.noise(rng, issued, INT_DCI, 8);
        let batch = drain_events(self.fx.intr(), &mut self.fx.ring);
        assert!(batch.count <= DRAIN_BATCH);
        for t in &batch.trbs[..batch.count] {
            assert!(!is_poison(t), "the drain read past the segment");
        }
    }

    fn poll(&mut self, rng: &mut Rng) {
        let length = rng.below(HID_REPORT_MAX as u64 + 1) as u16;
        let ep = &self.hid.interrupt[0];
        let issued = ep.armed.unwrap_or(ep.ring.enqueue_phys());
        self.noise(rng, issued, INT_DCI, length as u32);
        if rng.coin() {
            let event = genuine(rng, issued, INT_DCI, length as u32);
            self.fx.hc.post(event);
        }
        let got = poll_interrupt_in(
            self.fx.db(),
            self.fx.intr(),
            &mut self.fx.ring,
            &mut self.hid,
            INT_DCI,
            length,
        );
        match got {
            Ok(IntrPoll::Complete(n)) => {
                assert!(n <= length, "{n} bytes handed up for {length}");
                assert!(n as usize <= HID_REPORT_MAX);
            }
            Ok(IntrPoll::Pending) => {}
            // A report that ended in an error code is reported as that error
            // and disarms the endpoint, which the server then recovers.
            Err(XhciError::TransferCompletionFailed(_)) => {
                assert!(self.hid.interrupt[0].armed.is_none(), "a failed report stays armed");
            }
            Err(e) => panic!("the poll failed: {e:?}"),
        }
    }

    fn bulk(&mut self, rng: &mut Rng) {
        let len = rng.below(BULK_MAX as u64) as u32 + 1;
        let issued = self.pipes.in_ring.enqueue_phys();
        self.noise(rng, issued, BULK_IN_DCI, len);
        let event = genuine(rng, issued, BULK_IN_DCI, len);
        self.fx.hc.post(event);
        let moved = bulk_transfer(
            self.fx.db(),
            self.fx.intr(),
            &mut self.fx.ring,
            &mut self.pipes,
            SLOT,
            true,
            len,
        );
        match moved {
            Ok(n) => assert!(n <= len && n as usize <= BULK_MAX, "{n} bytes moved for {len}"),
            Err(XhciError::TransferCompletionFailed(_)) => {}
            Err(e) => panic!("the bulk transfer failed: {e:?}"),
        }
    }

    fn control(&mut self, rng: &mut Rng) {
        let at = (self.ep0.enqueue_phys() - self.ep0.phys()) / TRB;
        let status = self.ep0.phys() + ((at + 2) % RING_TRBS) * TRB;
        self.noise(rng, status, EP0_DCI, DEVICE_DESCRIPTOR_LEN as u32);
        let event = genuine(rng, status, EP0_DCI, DEVICE_DESCRIPTOR_LEN as u32);
        self.fx.hc.post(event);
        let got = get_device_descriptor(
            self.fx.db(),
            self.fx.intr(),
            &mut self.fx.ring,
            SLOT,
            &mut self.ep0,
            &self.desc,
        );
        match got {
            Ok(n) => assert_eq!(n, DEVICE_DESCRIPTOR_LEN as usize),
            Err(XhciError::TransferCompletionFailed(_)) => {}
            Err(e) => panic!("the control transfer failed: {e:?}"),
        }
    }

    fn enable_slot(&mut self, rng: &mut Rng) {
        let issued = self.next_command();
        self.noise(rng, issued, EP0_DCI, 0);
        let slot = rng.next() as u8;
        self.fx.hc.post(command(issued, CC_SUCCESS, slot));
        let got = issue_enable_slot(self.fx.db(), self.fx.intr(), &mut self.cmd, &mut self.fx.ring);
        self.cmd_issued += 1;
        match got {
            Ok(slot) => {
                assert_ne!(slot, 0, "slot 0 is never handed out");
                let taken = self.table.mark_allocated(slot, MAX_SLOTS);
                assert_eq!(taken, slot <= MAX_SLOTS, "slot {slot} against MaxSlots {MAX_SLOTS}");
                assert_eq!(self.table.mark_released(slot, MAX_SLOTS), taken);
            }
            Err(XhciError::ControllerUnsupported | XhciError::CommandCompletionFailed(_)) => {}
            Err(e) => panic!("enable slot failed: {e:?}"),
        }
    }

    fn check(&self) {
        let fx = &self.fx;
        let dequeue = fx.dequeue_bus();
        let erdp = fx.hc.erdp();
        assert_eq!(erdp & 0x7, 0, "DESI names the only segment");
        assert_eq!(erdp & !ERDP_FLAGS, dequeue, "ERDP names the next unread TRB");
        if fx.ring.drained_total() > 0 {
            assert_ne!(erdp & ERDP_EHB_BIT, 0, "EHB is cleared with every hand-back");
        }
    }
}

#[test]
fn hostile_events_never_panic_or_reach_past_a_ring_or_a_buffer() {
    let mut driver = Driver::new();
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for _ in 0..ROUNDS {
        driver.round(&mut rng);
    }
    let laps = driver.fx.ring.drained_total() / driver.fx.hc.segment_trbs() as u64;
    assert!(laps > 1000, "the rounds went round the ring {laps} times");
}
