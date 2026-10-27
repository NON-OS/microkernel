// NONOS Operating System (AGPL-3.0-or-later)

//! The TY and MA images booted against the modeled V1 device. The discrete
//! AX210 sets its boot LTR in CSR_LTR_LONG_VAL_AD before UREG_CPU_INIT_RUN
//! and keeps nothing busy (`iwl_pcie_set_ltr`, Linux v6.17 `iwl_ty_mac_cfg`);
//! Meteor Lake, integrated, clears the image-loader cause instead and spins
//! for it. Given its SKU at ALIVE, each takes its own file's PNVM section.

use super::rig::TY_REV;
use super::rig::{polls, rig, run, GF, GF4, MA, MA4, MA4_PNVM, MA_PNVM, MA_REV, TY, TY_PNVM};
use crate::gen3::pnvm::parse;
use crate::gen3::regs::UREG_CPU_INIT_RUN;
use crate::gen3::regs::{CSR_MSIX_HW_INT_CAUSES_AD, HBUS_TARG_PRPH_WADDR, MSIX_HW_IML};
use crate::gen3::start::{CSR_LTR_LONG_VAL_AD, LTR_BOOT};

// The first SKU a PNVM file names (its first SKU TLV, type 64).
fn first_sku(f: &[u8]) -> [u32; 3] {
    let w = |o: usize| u32::from_le_bytes(f[o..o + 4].try_into().unwrap());
    let mut at = 0;
    while w(at) != 64 {
        at += 8 + (w(at + 4) as usize).div_ceil(4) * 4;
    }
    [w(at + 8), w(at + 12), w(at + 16)]
}

#[test]
fn the_boot_ltr_is_250_us_snooped_and_not() {
    assert_eq!(LTR_BOOT, 0x88FA_88FA);
}

#[test]
fn a_discrete_ax210_sets_its_boot_ltr_and_does_not_spin() {
    for discrete in [true, false] {
        let mut r = rig(TY, TY_PNVM, TY_REV, GF);
        r.board.discrete = discrete;
        let mut c = polls();
        assert_eq!(run(&r, &mut c).0.expect("alive").status, 0xCAFE);
        let s = r.model.s.borrow();
        let at = |f: &dyn Fn(&(usize, u32)) -> bool| s.writes.iter().position(f);
        let kick = at(&|&(o, v)| {
            o == HBUS_TARG_PRPH_WADDR && v & 0xFF_FFFF == UREG_CPU_INIT_RUN + 0x30_0000
        });
        let ltr = at(&|&(o, v)| o == CSR_LTR_LONG_VAL_AD && v == LTR_BOOT);
        let iml = at(&|&(o, v)| o == CSR_MSIX_HW_INT_CAUSES_AD && v == MSIX_HW_IML);
        let kick = kick.expect("UREG_CPU_INIT_RUN");
        if discrete {
            assert!(ltr.expect("the boot LTR") < kick && iml.is_none(), "discrete");
        } else {
            assert!(iml.expect("the IML cause cleared") < kick && ltr.is_none(), "integrated");
        }
        // The integrated part spins up to 100 ms for the image loader.
        assert_eq!(c.delays_us >= 100_000, !discrete, "{discrete}");
    }
}

#[test]
fn each_image_gets_its_own_pnvm_section_for_its_sku() {
    for (blob, pnvm, rev, rf) in
        [(TY, TY_PNVM, TY_REV, GF), (MA, MA_PNVM, MA_REV, GF), (MA4, MA4_PNVM, MA_REV, GF4)]
    {
        let r = rig(blob, pnvm, rev, rf);
        let sku = first_sku(pnvm);
        r.model.s.borrow_mut().alive_sku = sku;
        let (alive, given) = run(&r, &mut polls());
        alive.expect("alive, and the PNVM step done");
        let want = parse(pnvm, sku, (rev >> 4) as u16, 0x10D).expect("a section").version;
        assert_eq!(given, Some(want), "{rev:#x} {rf:#x}");
    }
}
