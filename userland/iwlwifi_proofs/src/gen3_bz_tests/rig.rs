// NONOS Operating System (AGPL-3.0-or-later)

//! The bundled BZ-family files, and a modeled V2 device booting one of them.

use std::rc::Rc;

use crate::gen3::alive::Alive;
use crate::gen3::bringup::{boot, BootError};
use crate::gen3::csr::CsrLayout;
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::plan::{control_len, RB_REGION};
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{Mem, Model, Polls};

macro_rules! intel {
    ($name:literal) => {
        include_bytes!(concat!("../../../../nonos-bootloader/firmware/intel/", $name))
    };
}
pub(super) static GL: &[u8] = intel!("iwlwifi-gl-c0-fm-c0-92.ucode");
pub(super) static GL_PNVM: &[u8] = intel!("iwlwifi-gl-c0-fm-c0.pnvm");
pub(super) static BZ: &[u8] = intel!("iwlwifi-bz-b0-fm-c0-92.ucode");
pub(super) static BZ_PNVM: &[u8] = intel!("iwlwifi-bz-b0-fm-c0.pnvm");

/// CSR_HW_REV and CSR_HW_RF_ID of a BE200 (Gl C0, FM C0) as Linux decodes
/// them; the BE201's hw_rev has its step from the CNVi ORed in.
pub(super) const GL_REV: u32 = 0x0000_0472;
pub(super) const BZ_REV: u32 = 0x0000_0461;
pub(super) const FM_C: u32 = 0x0011_2200;

pub(super) struct Rig {
    pub(super) model: Model,
    pub(super) fw: Vec<Rc<Mem>>,
    pub(super) blob: &'static [u8],
}

/// A V2 device (or a V1 one, `v2` false) and regions for `blob`.
pub(super) fn rig(blob: &'static [u8], v2: bool) -> Rig {
    let ucode = Ucode::parse(blob).unwrap();
    let ctrl = Mem::new(control_len(ucode.iml.len()).unwrap(), 0x1000_0000);
    let rbs = Mem::new(RB_REGION, 0x2000_0000);
    let sizes = firmware_region_sizes(&classify(blob)).unwrap();
    let fw = sizes.iter().enumerate();
    let fw = fw.map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000)).collect();
    let model = Model::new(ctrl, rbs);
    model.s.borrow_mut().v2 = v2;
    Rig { model, fw, blob }
}

pub(super) fn polls() -> Polls {
    Polls { per_wait: 64, delays_us: 0 }
}

/// Boot the rig's image in layout `csr`, then run `then` on the device.
pub(super) fn run<T>(
    r: &Rig,
    csr: CsrLayout,
    c: &mut Polls,
    then: impl FnOnce(&mut Dev<'_, Model, Mem>, &mut Polls, &Ucode<'_>) -> T,
) -> (Result<Alive, BootError>, Option<T>) {
    let ucode = Ucode::parse(r.blob).unwrap();
    let layout = classify(r.blob);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let board = Board { hw_rev: GL_REV, imr_enabled: false, rf_id: FM_C, pnvm: None };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref()).with_csr(csr);
    let alive = boot(&mut dev, c, &mem, &layout, &ucode, &board);
    let after = alive.is_ok().then(|| then(&mut dev, c, &ucode));
    (alive, after)
}
