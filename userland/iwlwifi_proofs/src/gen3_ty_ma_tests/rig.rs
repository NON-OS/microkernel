// NONOS Operating System (AGPL-3.0-or-later)

//! The carried discrete-AX210 and Meteor Lake files, and a modeled V1 device
//! booting one of them as the radio does.

use std::rc::Rc;

use crate::gen3::alive::Alive;
use crate::gen3::bringup::{boot, BootError};
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::plan::{control_len_pnvm, RB_REGION};
use crate::gen3::pnvm::reserve;
use crate::gen3::select::{mac_type, rf_type};
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{Mem, Model, Polls};

macro_rules! intel {
    ($name:literal) => {
        include_bytes!(concat!("../../../../nonos-bootloader/firmware/intel/", $name))
    };
}
pub(super) static SO: &[u8] = intel!("iwlwifi-so-a0-gf-a0-86.ucode");
pub(super) static TY: &[u8] = intel!("iwlwifi-ty-a0-gf-a0-86.ucode");
pub(super) static TY_PNVM: &[u8] = intel!("iwlwifi-ty-a0-gf-a0.pnvm");
pub(super) static MA: &[u8] = intel!("iwlwifi-ma-b0-gf-a0-86.ucode");
pub(super) static MA_PNVM: &[u8] = intel!("iwlwifi-ma-b0-gf-a0.pnvm");
pub(super) static MA4: &[u8] = intel!("iwlwifi-ma-b0-gf4-a0-86.ucode");
pub(super) static MA4_PNVM: &[u8] = intel!("iwlwifi-ma-b0-gf4-a0.pnvm");

/// CSR_HW_REV of a discrete AX210 (TY) and of Meteor Lake at B0; a GF radio
/// at step A, and the same with the CDB bit (GF4).
pub(super) const TY_REV: u32 = 0x0000_0420;
pub(super) const MA_REV: u32 = 0x0000_0441;
pub(super) const GF: u32 = 0x0010_D000;
pub(super) const GF4: u32 = 0x1010_D000;

pub(super) struct Rig {
    pub(super) model: Model,
    pub(super) fw: Vec<Rc<Mem>>,
    pub(super) board: Board,
    blob: &'static [u8],
}

/// A V1 device and regions for `blob`, its control region reserving room
/// for the largest PNVM section `pnvm` holds for this MAC and RF.
pub(super) fn rig(blob: &'static [u8], pnvm: &'static [u8], hw_rev: u32, rf_id: u32) -> Rig {
    let ucode = Ucode::parse(blob).unwrap();
    let room = reserve(pnvm, mac_type(hw_rev), rf_type(rf_id));
    let ctrl = Mem::new(control_len_pnvm(ucode.iml.len(), room).unwrap(), 0x1000_0000);
    let sizes = firmware_region_sizes(&classify(blob)).unwrap();
    let fw = sizes.iter().enumerate();
    let fw = fw.map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000)).collect();
    let model = Model::new(ctrl, Mem::new(RB_REGION, 0x2000_0000));
    let board = Board { hw_rev, imr_enabled: false, rf_id, pnvm: Some(pnvm), discrete: false };
    Rig { model, fw, board, blob }
}

pub(super) fn polls() -> Polls {
    Polls { per_wait: 64, delays_us: 0 }
}

/// Boot the rig's image to ALIVE; the device's PNVM version comes back too.
pub(super) fn run(r: &Rig, c: &mut Polls) -> (Result<Alive, BootError>, Option<u32>) {
    let ucode = Ucode::parse(r.blob).unwrap();
    let layout = classify(r.blob);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    let alive = boot(&mut dev, c, &mem, &layout, &ucode, &r.board);
    (alive, dev.pnvm)
}
