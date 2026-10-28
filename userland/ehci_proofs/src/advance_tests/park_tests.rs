// NONOS Operating System (AGPL-3.0-or-later)
//! A periodic QH parked on a controller whose frames stop: the QH it may
//! still be running is not emptied. The controller is stopped as a dead
//! one is, and the QH emptied once it reads halted; one that will not stop
//! keeps the QH's overlay as it was. Either way the park fails.

use crate::env::Regs;
use crate::facts::qh::QH_LEN;
use crate::facts::regs::{FRINDEX, STS_HALTED, USBCMD};
use crate::model::{addressed, call, high, rig, ModelRegs, Shared, CAPLEN};
use crate::sched::advance::Dead;
use crate::sched::layout::intr_qh;
use crate::sched::park::park;
use crate::wire::ops::OP_ALLOC_TRANSFER_RING;

/// The model's registers with FRINDEX frozen; `stuck` drops every USBCMD
/// write too.
struct Frozen {
    inner: ModelRegs,
    stuck: bool,
}

impl Regs for Frozen {
    fn r32(&mut self, off: u32) -> u32 {
        if off == CAPLEN + FRINDEX {
            return 0x100;
        }
        self.inner.r32(off)
    }
    fn w32(&mut self, off: u32, v: u32) {
        if !(off == CAPLEN + USBCMD && self.stuck) {
            self.inner.w32(off, v);
        }
    }
}

/// A controller with an interrupt IN endpoint armed on periodic QH 0.
fn armed() -> (crate::model::Cx, Shared) {
    let (mut cx, w) = rig(vec![high(), None]);
    let slot = addressed(&mut cx, 1);
    let r = call(&mut cx, OP_ALLOC_TRANSFER_RING, &[slot, 0x81, 8, 0, 10, 0]);
    assert_eq!(r.status, 0);
    (cx, w)
}

fn overlay(w: &Shared) -> Vec<u8> {
    let at = intr_qh(0) + 12;
    w.borrow().mem[at..intr_qh(0) + QH_LEN].to_vec()
}

#[test]
fn frames_that_stop_on_a_controller_that_will_not_stop_leave_the_qh_alone() {
    let (cx, w) = armed();
    let before = overlay(&w);
    assert_ne!(before[4..8], [1, 0, 0, 0], "a qTD is armed");
    let mut r = Frozen { inner: ModelRegs(w.clone()), stuck: true };
    assert_eq!(park((&cx.mem, &mut r, &cx.clock), CAPLEN, intr_qh(0)), Err(Dead));
    assert_eq!(overlay(&w), before, "the armed qTD's overlay is not emptied");
    assert_eq!(w.borrow().sts & STS_HALTED, 0, "it would not stop");
}

#[test]
fn frames_that_stop_are_a_dead_controller_stopped_before_its_qh_is_emptied() {
    let (cx, w) = armed();
    let mut r = Frozen { inner: ModelRegs(w.clone()), stuck: false };
    assert_eq!(park((&cx.mem, &mut r, &cx.clock), CAPLEN, intr_qh(0)), Err(Dead));
    assert_ne!(w.borrow().sts & STS_HALTED, 0, "stopped");
    assert_eq!(overlay(&w)[4..8], [1, 0, 0, 0], "emptied once halted: no next qTD");
    let mut live = ModelRegs(w.clone());
    assert_eq!(park((&cx.mem, &mut live, &cx.clock), CAPLEN, intr_qh(0)), Ok(()), "halted");
}
