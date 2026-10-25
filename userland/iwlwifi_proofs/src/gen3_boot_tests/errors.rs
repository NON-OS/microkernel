// NONOS Operating System (AGPL-3.0-or-later)

//! A command without an answer, and a firmware error mid-sequence.

use super::rig::{board, polls, rig, so, SO_GF};
use crate::gen3::bringup::boot;
use crate::gen3::cmds::*;
use crate::gen3::dev::{Dev, WaitError};
use crate::gen3::dram_map::classify;
use crate::gen3::layout::Memory;
use crate::gen3::ucode::Ucode;
use crate::gen3::up::{up, UpError, SCAN_IF_ADDR};
use crate::gen3_model::{reply, Mem, Out, Seen};

#[test]
fn a_command_without_an_answer_names_itself() {
    let r = rig();
    r.model.s.borrow_mut().responder = Some(Box::new(|s: &Seen| {
        if (s.group, s.cmd) == (0x0C, 0x02) {
            vec![] // NVM_GET_INFO never answered
        } else if (s.group, s.cmd) == (0x0C, 0x00) {
            vec![reply(s, vec![]), Out { cmd: 0x04, group: 0, seq: 0x8000, payload: vec![] }]
        } else {
            vec![reply(s, vec![])]
        }
    }));
    let mut c = polls();
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &board()).unwrap();
    assert_eq!(
        up(&mut dev, &mut c, &ucode, &so(), SCAN_IF_ADDR),
        Err(UpError::Command(REGULATORY_AND_NVM_GROUP, NVM_GET_INFO, WaitError::TimedOut))
    );
}

#[test]
fn a_firmware_error_mid_sequence_is_reported_as_such() {
    let r = rig();
    r.model.s.borrow_mut().fw_error_after = Some(1);
    let mut c = polls();
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &board()).unwrap();
    assert_eq!(
        up(&mut dev, &mut c, &ucode, &so(), SCAN_IF_ADDR),
        Err(UpError::Command(REGULATORY_AND_NVM_GROUP, NVM_ACCESS_COMPLETE, WaitError::FirmwareError))
    );
}
