// NONOS Operating System (AGPL-3.0-or-later)
//! Bring-up: the controller run on the schedule the driver built.

use crate::facts::io::{CMD_CF, CMD_RS, STS_HCH};
use crate::hc::start::{start, HCRESET_STUCK};
use crate::model::{rig, ModelClock, ModelIo, ModelMem, World, BUS};
use crate::sched::layout::{FRAMES, INTR_QH};
use crate::server::context::Ctx;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn the_controller_runs_with_every_frame_on_the_first_interrupt_queue() {
    let (cx, w) = rig([None, None]);
    let w = w.borrow();
    assert_eq!(w.cmd & (CMD_RS | CMD_CF), CMD_RS | CMD_CF);
    assert_eq!(w.sts & STS_HCH, 0);
    assert_eq!(w.flbase, BUS);
    assert_eq!(w.intr, 0, "polled: no interrupt enabled");
    for f in 0..FRAMES as u32 {
        assert_eq!(w.rd32(BUS + 4 * f), (BUS + INTR_QH as u32) | 2);
    }
    drop(cx);
}

#[test]
fn frames_run_once_started_and_an_empty_schedule_ends_each() {
    let (cx, w) = rig([None, None]);
    let before = w.borrow().frames;
    for _ in 0..10 {
        crate::env::Clock::idle(&cx.clock);
    }
    assert_eq!(w.borrow().frames, before + 10);
}

#[test]
fn a_host_reset_that_never_clears_fails_the_start_within_its_bound() {
    let w = Rc::new(RefCell::new(World::default()));
    w.borrow_mut().hcreset_stuck = true;
    let mut cx = Ctx::new(ModelIo(w.clone()), ModelClock(w.clone()), ModelMem(w.clone()));
    assert_eq!(start(&mut cx.io, &cx.clock, &cx.mem), Err(HCRESET_STUCK));
    assert!(w.borrow().ms < 100, "bounded: {} ms", w.borrow().ms);
}
