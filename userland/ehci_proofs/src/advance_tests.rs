// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! A doorbell the controller never answers: the QH it may still hold is
//! not rewritten. The controller is stopped as a dead one is, and the QH
//! emptied once it reads halted; one that will not stop keeps the QH as
//! it was, linked back. Either way the take-back fails.

use crate::env::Regs;
use crate::facts::qh::QH_LEN;
use crate::facts::regs::{CMD_IAAD, STS_HALTED, USBCMD};
use crate::model::{addressed, bulk_ready, high, rig, ModelRegs, Shared, BUS, CAPLEN};
use crate::sched::advance::Dead;
use crate::sched::layout::{bulk_ring, qh, RING};
use crate::sched::take::take_back;

/// The model's registers with the doorbell never passed on: USBCMD reads
/// it set until it is cleared. `stuck` drops every USBCMD write too.
struct Deaf {
    inner: ModelRegs,
    iaad: bool,
    stuck: bool,
}

impl Regs for Deaf {
    fn r32(&mut self, off: u32) -> u32 {
        let held = off == CAPLEN + USBCMD && self.iaad;
        self.inner.r32(off) | if held { CMD_IAAD } else { 0 }
    }
    fn w32(&mut self, off: u32, v: u32) {
        let cmd = off == CAPLEN + USBCMD;
        self.iaad = if cmd { v & CMD_IAAD != 0 } else { self.iaad };
        if !(cmd && self.stuck) {
            self.inner.w32(off, if cmd { v & !CMD_IAAD } else { v });
        }
    }
}

fn qh_bytes(w: &Shared, at: usize) -> Vec<u8> {
    w.borrow().mem[at..at + QH_LEN].to_vec()
}

fn ring_whole(w: &Shared) -> bool {
    let w = w.borrow();
    let mut at = BUS + qh(0) as u32;
    (0..RING).for_each(|_| at = w.rd32(at) & !0x1F);
    at == BUS + qh(0) as u32
}

#[test]
fn a_controller_that_never_answers_and_will_not_stop_keeps_its_qh_untouched() {
    let (mut cx, w) = rig(vec![high(), None, None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    let (ring, before) = (bulk_ring(slot, true), qh_bytes(&w, qh(bulk_ring(slot, true))));
    let mut r = Deaf { inner: ModelRegs(w.clone()), iaad: false, stuck: true };
    let got = take_back((&cx.mem, &mut r, &cx.clock), CAPLEN, ring, false, Some((1, 2)));
    assert_eq!(got, Err(Dead));
    assert_eq!(qh_bytes(&w, qh(ring)), before, "not rewritten");
    assert!(ring_whole(&w) && w.borrow().sts & STS_HALTED == 0, "linked back, running");
}

#[test]
fn a_controller_that_never_answers_is_stopped_and_only_then_its_qh_rewritten() {
    let (mut cx, w) = rig(vec![high(), None, None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    let ring = bulk_ring(slot, true);
    let mut r = Deaf { inner: ModelRegs(w.clone()), iaad: false, stuck: false };
    let got = take_back((&cx.mem, &mut r, &cx.clock), CAPLEN, ring, false, Some((1, 2)));
    assert_eq!(got, Err(Dead));
    assert_ne!(w.borrow().sts & STS_HALTED, 0, "stopped");
    assert_eq!(w.borrow().rd32(BUS + qh(ring) as u32 + 4), 1, "rewritten once halted");
    assert!(ring_whole(&w));
}
