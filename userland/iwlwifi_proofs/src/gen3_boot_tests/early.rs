// NONOS Operating System (AGPL-3.0-or-later)

//! INIT_COMPLETE may come before the reply it follows in Linux's sequence:
//! ahead of NVM_ACCESS_COMPLETE's reply, or even with INIT_EXTENDED_CFG's.
//! The command drain sees it then, and the bring-up remembers it rather than
//! waiting for a second one that never comes.

use super::answers::firmware_answers;
use super::rig::{board, polls, rig, so, SO_GF};
use crate::gen3::bringup::boot;
use crate::gen3::cmds::*;
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::Memory;
use crate::gen3::ucode::Ucode;
use crate::gen3::up::{up, SCAN_IF_ADDR};
use crate::gen3_model::{Mem, Out, Seen};

const INIT_COMPLETE: Out = Out { cmd: 0x04, group: 1, seq: 0x8000, payload: Vec::new() };

#[test]
fn an_init_complete_seen_while_a_command_drains_is_not_waited_for_again() {
    // (group, cmd) of the command whose reply INIT_COMPLETE goes ahead of.
    for early in [(0x0C, 0x00), (0x02, 0x03)] {
        let r = rig();
        let ucode = Ucode::parse(SO_GF).unwrap();
        let mut base = firmware_answers(ucode.capa(CAPA_MCC_UPDATE_11AX_SUPPORT));
        r.model.s.borrow_mut().responder = Some(Box::new(move |s: &Seen| {
            let mut out: Vec<Out> =
                base(s).into_iter().filter(|o| (o.group, o.cmd) != (1, 0x04)).collect();
            if (s.group, s.cmd) == early {
                out.insert(0, INIT_COMPLETE);
            }
            out
        }));
        let mut c = polls();
        let layout = classify(SO_GF);
        let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
        let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
        let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
        boot(&mut dev, &mut c, &mem, &layout, &ucode, &board()).expect("alive");
        let before = c.delays_us;
        up(&mut dev, &mut c, &ucode, &so(), SCAN_IF_ADDR).expect("up");
        assert!(c.delays_us - before < 100_000, "no wait ran out: {early:?}");
    }
}
