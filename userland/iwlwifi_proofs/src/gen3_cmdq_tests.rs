// NONOS Operating System (AGPL-3.0-or-later)
//! W4: the command buffer is one, so a command the firmware never answered
//! keeps it: the next command is refused, and none reaches the device, until
//! the late reply retires it (or the serving loop restarts the firmware).

use crate::gen3::bringup::boot;
use crate::gen3::cmdq::CmdQueue;
use crate::gen3::dev::{Dev, Flow, WaitError};
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::outcome::{Failure, STAGE_FIRMWARE_FAILED, STEP_UNANSWERED};
use crate::gen3::packet::Packet;
use crate::gen3::plan::{control_len, RB_REGION};
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{reply, Mem, Model, Polls};

static SO_GF: &[u8] =
    include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

#[test]
fn an_unanswered_command_keeps_the_buffer_until_its_late_reply() {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let ctrl = Mem::new(control_len(ucode.iml.len()).unwrap(), 0x1000_0000);
    let model = Model::new(ctrl, Mem::new(RB_REGION, 0x2000_0000));
    let sizes = firmware_region_sizes(&layout).unwrap();
    let fw: Vec<_> = sizes.iter().map(|&n| Mem::new(n, 0x4000_0000)).collect();
    let fw: Vec<&Mem> = fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: model.ctrl.as_ref(), rbs: model.rbs.as_ref(), fw: &fw };
    let board = Board { hw_rev: 0x370, imr_enabled: false, rf_id: 0x10D000, pnvm: None };
    let mut c = Polls { per_wait: 64, delays_us: 0 };
    let mut dev = Dev::new(&model, model.ctrl.as_ref(), model.rbs.as_ref());
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &board).expect("alive");
    model.s.borrow_mut().responder = Some(Box::new(|_| Vec::new()));
    let (mut quiet, mut other) = (|_: &[u8]| {}, |_: &Packet<'_>| {});
    let sent = model.s.borrow().commands.len();
    let got = dev.command(&mut c, (0x1, 0x2), &[7], &mut quiet, &mut other);
    assert_eq!(got, Err(WaitError::TimedOut));
    let got = dev.command(&mut c, (0x1, 0x3), &[8], &mut quiet, &mut other);
    assert_eq!(got, Err(WaitError::Busy), "its buffer may still be read");
    assert_eq!(model.s.borrow().commands.len(), sent + 1, "the second never reached it");
    let first = model.s.borrow().commands[sent].clone();
    model.s.borrow_mut().responder = None;
    model.queue(reply(&first, vec![]));
    assert_eq!(dev.drain(&mut |_| Flow::Continue), Flow::Continue);
    assert!(!dev.cmdq.busy(), "the late reply retired it");
    assert!(dev.command(&mut c, (0x1, 0x3), &[8], &mut quiet, &mut other).is_ok());
}

#[test]
fn only_the_reply_to_the_command_in_flight_retires_it() {
    let mut q = CmdQueue::new();
    assert!(!q.busy());
    q.mark(0x0005);
    q.retire(0x0006);
    assert!(q.busy(), "another sequence retires nothing");
    q.retire(0x0005);
    assert!(!q.busy());
    let f = Failure::Unanswered;
    assert_eq!((f.stage(), f.step(), f.detail()), (STAGE_FIRMWARE_FAILED, STEP_UNANSWERED, 0));
    assert_eq!(f.text(), "a firmware command went unanswered");
}
