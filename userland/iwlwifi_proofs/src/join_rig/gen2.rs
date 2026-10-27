// NONOS Operating System (AGPL-3.0-or-later)

//! The rig as a 22000-family part: an AX201 CNVi on a Qu MAC at step B with
//! an HR2 radio, booting Qu-b0-hr-b0 through the version 1 context info, its
//! rings, packets and transmit commands this family's, the same access point
//! and responder answering it.

use super::*;
use crate::gen3::csr::CsrLayout;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board};
use crate::gen3::plan::control_len;
use crate::gen3_model::{Mem, Model};

pub static QU_B0_HR: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-Qu-b0-hr-b0-77.ucode");

/// The rig of `rig`, its device a Qu B-step part with an HR2 radio.
pub fn rig_gen2(sec: Security, sw: Switches) -> Rig {
    let mut r = rig(sec, sw);
    let ctrl = Mem::new(control_len(0).unwrap(), 0x1000_0000);
    let model = Model::new(ctrl, Mem::new(CsrLayout::Gen2.rb_region(), 0x2000_0000));
    {
        let mut old = r.model.s.borrow_mut();
        let mut s = model.s.borrow_mut();
        (s.txq, s.responder, s.gen2) = (old.txq.take(), old.responder.take(), true);
    }
    let sizes = firmware_region_sizes(&classify(QU_B0_HR)).unwrap();
    let fw = sizes.iter().enumerate();
    r.fw_regions = fw.map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000)).collect();
    (r.model, r.gen2) = (model, true);
    r
}

/// What `with_fw` boots: the image, the family's layout and the board.
pub(super) fn part(r: &Rig) -> (&'static [u8], CsrLayout, Board) {
    let board =
        |hw_rev, rf_id| Board { hw_rev, imr_enabled: false, rf_id, pnvm: None, discrete: false };
    match r.gen2 {
        true => (QU_B0_HR, CsrLayout::Gen2, board(0x331, 0x10A100)),
        false => (SO_GF, CsrLayout::V1, board(0x370, 0x10D000)),
    }
}
