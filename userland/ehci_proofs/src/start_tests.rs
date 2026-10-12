// NONOS Operating System (AGPL-3.0-or-later)
//! Bring-up: reset, both schedules on the lists the driver built, every
//! port routed here and powered.

use std::cell::RefCell;
use std::rc::Rc;

use crate::facts::port::PORT_PP;
use crate::facts::qh::CH_H;
use crate::facts::regs::*;
use crate::hc::start::{start, HCRESET_STUCK};
use crate::model::{rig, ModelClock, ModelMem, ModelRegs, World, BUS, CAPLEN, PORTS};
use crate::sched::layout::{intr_qh, qh, FRAMES, RING};
use crate::server::context::Ctx;

#[test]
fn the_controller_runs_both_schedules_with_every_port_routed_and_powered() {
    let (cx, w) = rig(vec![None, None, None]);
    let w = w.borrow();
    assert_eq!((cx.table.op, cx.table.ports as usize), (CAPLEN, PORTS));
    assert_eq!(w.sts & (STS_HALTED | STS_PSS | STS_ASS), STS_PSS | STS_ASS);
    assert_eq!(w.configflag, CF_EHCI);
    assert!(w.ports.iter().all(|p| p & PORT_PP != 0));
    for f in 0..FRAMES as u32 {
        assert_eq!(w.rd32(w.periodic + 4 * f), (BUS + intr_qh(0) as u32) | 2);
    }
    assert_eq!(w.rd32(BUS + intr_qh(0) as u32 + 8) & 0xFF, 0, "no interrupt QH runs yet");
}

#[test]
fn the_async_ring_closes_on_its_head_the_one_qh_marked_h() {
    let (_cx, w) = rig(vec![None, None, None]);
    let w = w.borrow();
    assert_eq!(w.async_addr, BUS + qh(0) as u32);
    let mut at = w.async_addr;
    let mut heads = 0;
    for _ in 0..RING {
        heads += (w.rd32(at + 4) & CH_H != 0) as u32;
        at = w.rd32(at) & !0x1F;
    }
    assert_eq!((at, heads), (w.async_addr, 1));
}

#[test]
fn a_host_reset_that_never_clears_fails_the_start_within_its_bound() {
    let w = Rc::new(RefCell::new(World { hcreset_stuck: true, ..World::new() }));
    let mut cx = Ctx::new(ModelRegs(w.clone()), ModelClock(w.clone()), ModelMem(w.clone()));
    assert_eq!(start(&mut cx.io, &cx.clock, &cx.mem), Err(HCRESET_STUCK));
    assert!(w.borrow().ms <= 251, "{} ms", w.borrow().ms);
}
