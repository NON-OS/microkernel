// NONOS Operating System (AGPL-3.0-or-later)

//! The carried 22000-family images, and the modeled device as a 22000-family
//! part booting one of them.

use std::rc::Rc;

use crate::gen3::alive::Alive;
use crate::gen3::bringup::{boot, BootError};
use crate::gen3::csr::CsrLayout;
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::plan::control_len;
use crate::gen3::region::Clock;
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{Mem, Model, Polls};

macro_rules! intel {
    ($name:literal) => {
        include_bytes!(concat!("../../../../nonos-bootloader/firmware/intel/", $name))
    };
}
pub(super) static QU_B: &[u8] = intel!("iwlwifi-Qu-b0-hr-b0-77.ucode");
pub(super) static QU_C: &[u8] = intel!("iwlwifi-Qu-c0-hr-b0-77.ucode");
pub(super) static QUZ: &[u8] = intel!("iwlwifi-QuZ-a0-hr-b0-77.ucode");
pub(super) static CC: &[u8] = intel!("iwlwifi-cc-a0-77.ucode");
pub(super) static SO: &[u8] = intel!("iwlwifi-so-a0-gf-a0-86.ucode");

/// CSR_HW_REV of a Qu at B0 and C0 and of a QuZ (B), and an HR2 radio.
pub(super) const QU_B0: u32 = 0x0000_0331;
pub(super) const QU_C0: u32 = 0x0000_0332;
pub(super) const QUZ_B: u32 = 0x0000_0351;
pub(super) const HR2: u32 = 0x0010_A100;
pub(super) const G2: CsrLayout = CsrLayout::Gen2;

pub(super) struct Rig {
    pub(super) model: Model,
    pub(super) fw: Vec<Rc<Mem>>,
    pub(super) board: Board,
    pub(super) blob: &'static [u8],
}

/// A 22000-family device and regions for `blob` on a board of `hw_rev`.
pub(super) fn rig(blob: &'static [u8], hw_rev: u32, discrete: bool) -> Rig {
    let ctrl = Mem::new(control_len(0).unwrap(), 0x1000_0000);
    let sizes = firmware_region_sizes(&classify(blob)).unwrap();
    let fw = sizes.iter().enumerate();
    let fw = fw.map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000)).collect();
    let model = Model::new(ctrl, Mem::new(G2.rb_region(), 0x2000_0000));
    // A part fresh from another boot: CSR_CTXT_INFO_BA's high half is stale.
    (model.s.borrow_mut().gen2, model.s.borrow_mut().ctxt_ba) = (true, 0xDEAD << 48);
    let board = Board { hw_rev, imr_enabled: false, rf_id: HR2, pnvm: None, discrete };
    Rig { model, fw, board, blob }
}

pub(super) fn polls() -> Polls {
    Polls { per_wait: 64, delays_us: 0 }
}

/// Boot the rig's image to ALIVE in the Gen2 layout, then run `then`.
pub(super) fn run<T, C: Clock>(
    r: &Rig,
    c: &mut C,
    then: impl FnOnce(&mut Dev<'_, Model, Mem>, &mut C, &Ucode<'_>) -> T,
) -> (Result<Alive, BootError>, Option<T>) {
    let ucode = Ucode::parse_for(r.blob, G2).unwrap();
    let layout = classify(r.blob);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref()).with_csr(G2);
    let alive = boot(&mut dev, c, &mem, &layout, &ucode, &r.board);
    let after = alive.is_ok().then(|| then(&mut dev, c, &ucode));
    (alive, after)
}
