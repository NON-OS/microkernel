// NONOS Operating System (AGPL-3.0-or-later)

//! The modeled device and the bundled image the bring-up runs on.

use std::rc::Rc;

use crate::gen3::bringup::{boot, BootError};
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::plan::{control_len, RB_REGION};
use crate::gen3::select::{transport, Transport};
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{Mem, Model, Polls};

pub(super) static SO_GF: &[u8] = include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

pub(super) struct Rig {
    pub(super) model: Model,
    pub(super) fw: Vec<Rc<Mem>>,
}

pub(super) fn rig() -> Rig {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let ctrl = Mem::new(control_len(ucode.iml.len()).unwrap(), 0x1000_0000);
    let rbs = Mem::new(RB_REGION, 0x2000_0000);
    let layout = classify(SO_GF);
    let sizes = firmware_region_sizes(&layout).unwrap();
    let fw = sizes.iter().enumerate().map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000)).collect();
    Rig { model: Model::new(ctrl, rbs), fw }
}

pub(super) fn board() -> Board {
    Board { hw_rev: 0x0000_0370, imr_enabled: false, rf_id: 0x0010_D000, pnvm: None }
}

pub(super) fn so() -> Transport {
    transport(0x51F0).unwrap()
}

pub(super) fn run_boot(r: &Rig, c: &mut Polls) -> Result<crate::gen3::alive::Alive, BootError> {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    boot(&mut dev, c, &mem, &layout, &ucode, &board())
}

pub(super) fn polls() -> Polls {
    Polls { per_wait: 64, delays_us: 0 }
}
