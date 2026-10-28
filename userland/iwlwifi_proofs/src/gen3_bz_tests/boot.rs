// NONOS Operating System (AGPL-3.0-or-later)

//! The BE200 image booted in the V2 layout against a device that answers V2
//! bits only: the software reset in CSR_GP_CNTRL, D0A through MAC_INIT and
//! MAC_STATUS, CSR_FUNC_SCRATCH then ROM_START (no UREG_CPU_INIT_RUN), the
//! receive index through HBUS_TARG_WRPTR, ALIVE, then the post-ALIVE
//! commands in Linux's order with the v8 MCC reply.

use super::super::gen3_boot_tests::answers::firmware_answers;
use super::rig::{polls, rig, run, BZ, GL};
use crate::gen3::cmds::{LONG_GROUP, MAC_CONFIG_CMD, MAC_CONF_GROUP, MCC_UPDATE_CMD};
use crate::gen3::csr::{CsrLayout, CSR_FUNC_SCRATCH, FUNC_SCRATCH_INIT, GP_MAC_INIT};
use crate::gen3::csr::{GP_ROM_START, GP_SW_RESET, WRPTR_RX_Q0};
use crate::gen3::regs::{CSR_GP_CNTRL, CSR_RESET, HBUS_TARG_WRPTR, RESET_SW_RESET};
use crate::gen3::regs::{RFH_Q0_FRBDCB_WIDX_TRG, UREG_CPU_INIT_RUN};
use crate::gen3::select::transport;
use crate::gen3::up::{up, SCAN_IF_ADDR};
use crate::gen3_model::{reply, Seen};

#[test]
fn the_be200_and_be201_images_boot_to_alive_in_the_v2_sequence() {
    for blob in [GL, BZ] {
        let r = rig(blob, true);
        let (alive, _) = run(&r, CsrLayout::V2, &mut polls(), |_, _, _| ());
        assert_eq!(alive.expect("alive").status, 0xCAFE);
        let s = r.model.s.borrow();
        let w = &s.writes;
        let at = |f: &dyn Fn(&(usize, u32)) -> bool| w.iter().position(f);
        let gp = |bit: u32| move |&(o, v): &(usize, u32)| o == CSR_GP_CNTRL && v & bit != 0;
        let reset = at(&gp(GP_SW_RESET)).expect("SW_RESET in CSR_GP_CNTRL");
        let init = at(&gp(GP_MAC_INIT)).expect("MAC_INIT");
        let scratch = at(&|&(o, v)| o == CSR_FUNC_SCRATCH && v == FUNC_SCRATCH_INIT).unwrap();
        let rom = at(&gp(GP_ROM_START)).expect("ROM_START");
        assert!(reset < init && init < scratch && scratch < rom, "reset, D0A, scratch, kick");
        assert!(!w.iter().any(|&(o, v)| o == CSR_RESET && v & RESET_SW_RESET != 0));
        assert!(!s.prph.iter().any(|&(a, _)| a == UREG_CPU_INIT_RUN + 0x30_0000), "no UREG kick");
        assert!(w.iter().any(|&(o, v)| o == HBUS_TARG_WRPTR && v & 0xFFFF_0000 == WRPTR_RX_Q0));
        assert!(!w.iter().any(|&(o, _)| o == RFH_Q0_FRBDCB_WIDX_TRG), "no V1 free index");
    }
}

#[test]
fn after_alive_the_be200_comes_up_with_the_v8_regulatory_reply() {
    let r = rig(GL, true);
    let mut base = firmware_answers(true);
    r.model.s.borrow_mut().responder = Some(Box::new(move |s: &Seen| {
        if (s.group, s.cmd) != (LONG_GROUP, MCC_UPDATE_CMD) {
            return base(s);
        }
        // `iwl_mcc_update_resp_v8`: mcc 4, cap 8, n_channels 20, none.
        let mut p = vec![0u8; 24];
        p[4..6].copy_from_slice(&0x5553u16.to_le_bytes());
        vec![reply(s, p)]
    }));
    let t = transport(0x272B).unwrap();
    let (alive, info) =
        run(&r, CsrLayout::V2, &mut polls(), |dev, c, u| up(dev, c, u, &t, SCAN_IF_ADDR));
    alive.expect("alive");
    let info = info.unwrap().expect("up");
    assert_eq!(info.mcc, Some(0x5553));
    let cmds = r.model.s.borrow().commands.clone();
    let mac = cmds.iter().find(|s| (s.group, s.cmd) == (MAC_CONF_GROUP, MAC_CONFIG_CMD));
    assert_eq!(&mac.unwrap().payload[12..18], &SCAN_IF_ADDR);
    // `iwl_mvm_get_current_regdomain`: "ZZ" from MCC_SOURCE_GET_CURRENT,
    // the image taking Wi-Fi MCC updates (API 9).
    let mcc = cmds.iter().find(|s| (s.group, s.cmd) == (LONG_GROUP, MCC_UPDATE_CMD));
    assert_eq!(mcc.unwrap().payload[..3], [b'Z', b'Z', 0x10]);
}
