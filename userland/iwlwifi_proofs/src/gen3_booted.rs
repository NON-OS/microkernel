// NONOS Operating System (AGPL-3.0-or-later)
//! The gen3 model booted to ALIVE on the bundled SO image, for proofs that
//! drive the running transport.

use crate::gen3::bringup::boot;
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::plan::{control_len, RB_REGION};
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{reply, Mem, Model, Polls};

static SO_GF: &[u8] =
    include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

// Boot the model, answer every command with an empty reply, and run `f`.
pub fn booted(f: impl FnOnce(&Model, &mut Dev<'_, Model, Mem>, &mut Polls)) {
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
    model.s.borrow_mut().responder = Some(Box::new(|s| vec![reply(s, vec![])]));
    f(&model, &mut dev, &mut c);
}
