// NONOS Operating System (AGPL-3.0-or-later)

//! After ALIVE the commands go in Linux's order and a scan hands up beacons.

use super::answers::firmware_answers;
use super::rig::{board, polls, rig, so, SO_GF};
use crate::gen3::bringup::boot;
use crate::gen3::cmds::*;
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::Memory;
use crate::gen3::sweep::{Sweep, Tick};
use crate::gen3::ucode::Ucode;
use crate::gen3::up::{up, SCAN_IF_ADDR};
use crate::gen3_model::Mem;

#[test]
fn after_alive_the_commands_go_in_linux_order_and_a_scan_hands_up_beacons() {
    let r2 = rig();
    let ucode = Ucode::parse(SO_GF).unwrap();
    let ax = ucode.capa(CAPA_MCC_UPDATE_11AX_SUPPORT);
    r2.model.s.borrow_mut().responder = Some(firmware_answers(ax));
    let mut c = polls();
    let layout = classify(SO_GF);
    let fw2: Vec<&Mem> = r2.fw.iter().map(|m| m.as_ref()).collect();
    let mem2 = Memory { ctrl: r2.model.ctrl.as_ref(), rbs: r2.model.rbs.as_ref(), fw: &fw2 };
    let mut dev2 = Dev::new(&r2.model, r2.model.ctrl.as_ref(), r2.model.rbs.as_ref());
    boot(&mut dev2, &mut c, &mem2, &layout, &ucode, &board()).expect("alive");
    let info = up(&mut dev2, &mut c, &ucode, &so(), SCAN_IF_ADDR).expect("up");
    assert_eq!(info.mcc, Some(0x5553));
    assert_eq!(info.nvm.channels.len(), 51, "LAR: every 2.4 and 5 GHz channel is scanned");

    let order: Vec<(u8, u8)> = r2.model.s.borrow().commands.iter().map(|s| (s.group, s.cmd)).collect();
    assert_eq!(
        order,
        vec![
            (SYSTEM_GROUP, INIT_EXTENDED_CFG_CMD),
            (REGULATORY_AND_NVM_GROUP, NVM_ACCESS_COMPLETE),
            (REGULATORY_AND_NVM_GROUP, NVM_GET_INFO),
            (LONG_GROUP, TX_ANT_CONFIGURATION_CMD),
            (LONG_GROUP, BT_CONFIG),
            (SYSTEM_GROUP, SOC_CONFIGURATION_CMD),
            (LONG_GROUP, POWER_TABLE_CMD),
            (LONG_GROUP, MCC_UPDATE_CMD),
            (LONG_GROUP, SCAN_CFG_CMD),
            (MAC_CONF_GROUP, MAC_CONFIG_CMD),
            (MAC_CONF_GROUP, LINK_CONFIG_CMD),
        ],
        "the iwl_run_unified_mvm_ucode / iwl_mvm_up order"
    );
    let cmds = r2.model.s.borrow().commands.clone();
    assert_eq!(cmds[0].payload, vec![2, 0, 0, 0], "INIT_EXTENDED_CFG: BIT(IWL_INIT_NVM)");
    assert_eq!(cmds[3].payload, vec![3, 0, 0, 0], "TX antennas A and B");
    assert_eq!(&cmds[9].payload[12..18], &SCAN_IF_ADDR, "the scan interface address");
    // Sequences count up from zero in queue 0.
    assert!(cmds.iter().enumerate().all(|(i, s)| s.seq == i as u16), "sequence is the write index");

    let mut frames = Vec::new();
    let mut sweep = Sweep::new();
    let started = sweep.start(&mut dev2, &mut c, &info.nvm.channels, 0, &mut |f| frames.push(f));
    assert_eq!(started, Ok(Tick::Running), "the request is answered; the scan runs");
    assert_eq!(sweep.pump(&mut dev2, 10, &mut |f| frames.push(f)), Tick::Completed(1));
    assert!(!sweep.running());
    assert_eq!((sweep.completed, sweep.frames), (1, 1));
    assert_eq!(frames.len(), 1, "the beacon came up during the scan");
    assert_eq!(frames[0].channel, 6);
    assert_eq!(&frames[0].frame[frames[0].frame.len() - 4..], b"Home", "the MIC/CRC tail is cut");
    let req = r2.model.s.borrow().commands.last().unwrap().clone();
    assert_eq!((req.group, req.cmd, req.payload.len()), (LONG_GROUP, SCAN_REQ_UMAC, 1940));
    assert_eq!(u16::from_le_bytes([req.payload[8], req.payload[9]]) & (1 << 11), 1 << 11, "forced passive");
}
