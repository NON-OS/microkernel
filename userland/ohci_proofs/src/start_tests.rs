// NONOS Operating System (AGPL-3.0-or-later)
//! Bring-up: taken from firmware, reset, made operational on the lists the
//! driver built, and the root ports powered.

use std::cell::RefCell;
use std::rc::Rc;

use crate::facts::regs::*;
use crate::hc::own::SMM_KEPT;
use crate::hc::start::{start, HCR_STUCK};
use crate::model::{rig, ModelClock, ModelMem, ModelRegs, World, BUS, PORTS};
use crate::sched::layout::{bulk_ed, intr_ed, CTRL_ED, HCCA};
use crate::server::context::Ctx;

#[test]
fn the_controller_runs_every_list_with_every_frame_on_the_first_interrupt_ed() {
    let (cx, w) = rig(vec![None, None, None]);
    let w = w.borrow();
    let want = HCFS_OPERATIONAL | CTL_PLE | CTL_CLE | CTL_BLE;
    assert_eq!(w.control & (CTL_HCFS | CTL_PLE | CTL_CLE | CTL_BLE), want);
    assert_eq!((w.hcca, w.ctrl_head), (BUS + HCCA as u32, BUS + CTRL_ED as u32));
    assert_eq!(w.bulk_head, BUS + bulk_ed(0) as u32);
    assert_eq!(w.int_disabled & INT_ALL, INT_ALL, "polled: every interrupt off");
    for f in 0..32 {
        assert_eq!(w.rd32(w.hcca + 4 * f), BUS + intr_ed(0) as u32);
    }
    assert_eq!(w.fm_interval & FI_MASK, FI_DEFAULT);
    assert_eq!(w.periodic_start, FI_DEFAULT * 9 / 10);
    assert!(w.powered && w.legacy_off);
    assert_eq!(cx.table.ports as usize, PORTS);
}

fn bare(w: World) -> (Ctx<ModelRegs, ModelClock, ModelMem>, Rc<RefCell<World>>) {
    let w = Rc::new(RefCell::new(w));
    (Ctx::new(ModelRegs(w.clone()), ModelClock(w.clone()), ModelMem(w.clone())), w)
}

#[test]
fn firmwares_smm_driver_is_asked_for_the_controller_and_gives_it_up() {
    let (mut cx, w) = bare(World { smm: true, ..World::new() });
    assert_eq!(start(&mut cx.io, &cx.clock, &cx.mem, 0x1000), Ok(()));
    assert!(!w.borrow().smm);
}

#[test]
fn an_smm_driver_that_keeps_it_or_a_reset_that_sticks_fails_within_its_bound() {
    let (mut cx, w) = bare(World { smm: true, smm_deaf: true, ..World::new() });
    assert_eq!(start(&mut cx.io, &cx.clock, &cx.mem, 0x1000), Err(SMM_KEPT));
    assert!(w.borrow().ms <= 501, "{} ms", w.borrow().ms);
    let (mut cx, w) = bare(World { hcr_stuck: true, ..World::new() });
    assert_eq!(start(&mut cx.io, &cx.clock, &cx.mem, 0x1000), Err(HCR_STUCK));
    assert!(w.borrow().ms <= 11);
}

#[test]
fn legacy_emulation_is_left_alone_when_the_grant_does_not_reach_it() {
    let (mut cx, w) = bare(World::new());
    assert_eq!(start(&mut cx.io, &cx.clock, &cx.mem, 0x100), Ok(()));
    assert!(!w.borrow().legacy_off);
}
