// NONOS Operating System (AGPL-3.0-or-later)
//! The modeled device the radio proofs boot, and its clock.


use crate::gen3::bringup::boot;
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::plan::{control_len, RB_REGION};
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{Mem, Model, Polls};

pub(super) static SO_GF: &[u8] = include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

pub(super) struct Rig {
    pub(super) model: Model,
    pub(super) fw: Vec<std::rc::Rc<Mem>>,
}

pub(super) fn rig() -> Rig {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let ctrl = Mem::new(control_len(ucode.iml.len()).unwrap(), 0x1000_0000);
    let rbs = Mem::new(RB_REGION, 0x2000_0000);
    let sizes = firmware_region_sizes(&classify(SO_GF)).unwrap();
    let fw = sizes.iter().enumerate().map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000)).collect();
    Rig { model: Model::new(ctrl, rbs), fw }
}

pub(super) fn polls() -> Polls {
    Polls { per_wait: 64, delays_us: 0 }
}

// Boot the rig to ALIVE and run `f` with the live transport.
pub(super) fn booted(r: &Rig, f: impl FnOnce(&mut Dev<'_, Model, Mem>, &mut Polls)) {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    let mut c = polls();
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &Board { hw_rev: 0x370, imr_enabled: false, rf_id: 0x10D000, pnvm: None }).expect("alive");
    f(&mut dev, &mut c);
}
