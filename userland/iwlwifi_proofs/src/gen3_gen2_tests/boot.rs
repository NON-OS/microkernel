// NONOS Operating System (AGPL-3.0-or-later)

//! The 22000-family images booted against the modeled 22000 part: power
//! gating forced on the integrated part, the NIC configuration written, the
//! version 1 context info laid out and its address in CSR_CTXT_INFO_BA, the
//! LTR in the HPM registers (integrated) or CSR_LTR_LONG_VAL_AD (the
//! discrete AX200) before UREG_CPU_INIT_RUN, every peripheral address in the
//! 20-bit window with no UMAC offset, and no image-loader step.

use super::rig::{polls, rig, run, CC, QUZ, QUZ_B, QU_B, QU_B0, QU_C, QU_C0};
use crate::gen3::regs::{CSR_HW_IF_CONFIG_REG, CSR_MSIX_HW_INT_CAUSES_AD, MSIX_HW_IML};
use crate::gen3::start::gen2::{nic_config, NIC_CONFIG_MASK};
use crate::gen3::start::{CSR_LTR_LONG_VAL_AD, LTR_BOOT};
use crate::gen3::ucode::Ucode;

const CPU_INIT_RUN: u32 = 0x0_5C44;
const HPM_HIPM_GEN_CFG: u32 = 0x0_3458;

#[test]
fn each_ax201_image_boots_through_its_context_info() {
    for (blob, rev) in [(QU_B, QU_B0), (QU_C, QU_C0), (QUZ, QUZ_B)] {
        let r = rig(blob, rev, false);
        r.model.s.borrow_mut().regs.insert(CSR_HW_IF_CONFIG_REG, 0x00F0);
        assert_eq!(run(&r, &mut polls(), |_, _, _| ()).0.expect("alive").status, 0xCAFE);
        let s = r.model.s.borrow();
        let prph = &s.prph;
        assert!(prph.iter().all(|&(a, _)| a < 0x10_0000), "a 20-bit window, no UMAC offset");
        let gen_cfg: Vec<u32> =
            prph.iter().filter(|p| p.0 == HPM_HIPM_GEN_CFG).map(|p| p.1).collect();
        assert_eq!(gen_cfg, [1 << 10, 1 << 10 | 3, 3], "force active, gate, release");
        let at = |want: (u32, u32)| prph.iter().position(|&p| p == want).expect("written");
        let kick = at((CPU_INIT_RUN, 1));
        assert!(at((0x0_348C, 0xF)) < kick && at((0x0_3480, LTR_BOOT)) < kick, "the LTR first");
        let w = &s.writes;
        assert!(!w.iter().any(|&(o, _)| o == CSR_LTR_LONG_VAL_AD), "integrated: no CSR LTR");
        assert!(!w.iter().any(|&(o, v)| o == CSR_MSIX_HW_INT_CAUSES_AD && v == MSIX_HW_IML));
        assert_eq!(s.ctxt_ba, r.model.ctrl.dev, "CSR_CTXT_INFO_BA, both halves");
        let ucode = Ucode::parse_for(blob, super::rig::G2).unwrap();
        let cfg = w.iter().rfind(|&&(o, _)| o == CSR_HW_IF_CONFIG_REG).unwrap().1;
        assert_eq!(cfg & NIC_CONFIG_MASK, nic_config(rev, ucode.phy_config));
        assert_eq!(cfg & 0x00F0, 0x00F0, "bits it does not own kept");
    }
}

#[test]
fn the_nic_configuration_is_the_radio_and_mac_steps() {
    // The Qu images' PHY config 0x330018: radio type 0, step 2, dash 1.
    assert_eq!(nic_config(0x331, 0x0033_0018), 1 | 2 << 14 | 1 << 12);
    assert_eq!(nic_config(0x352, 0x0033_0003), 2 | 3 << 10);
}

#[test]
fn the_ax200_is_discrete_its_ltr_in_the_csr_and_its_gating_left_alone() {
    let r = rig(CC, 0x340, true);
    run(&r, &mut polls(), |_, _, _| ()).0.expect("alive");
    let s = r.model.s.borrow();
    assert!(!s.prph.iter().any(|p| p.0 == HPM_HIPM_GEN_CFG || p.0 == 0x0_3480));
    let ltr = s.writes.iter().position(|&(o, v)| o == CSR_LTR_LONG_VAL_AD && v == LTR_BOOT);
    let kick_waddr = s.writes.iter().position(|&(o, v)| o == 0x444 && v & 0xF_FFFF == CPU_INIT_RUN);
    assert!(ltr.expect("the boot LTR") < kick_waddr.expect("the kick"));
}
