// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The gen3 bring-up run end to end against the modeled device, with the
//! bundled so-a0-gf-a0-86 image: the start sequence, memory laid out across
//! broker-sized regions, the kick, ALIVE, the platform-NVM step, the
//! post-ALIVE commands in Linux's order, and a passive scan that hands up the
//! beacon the device received. Then each way it can fail: the NIC never
//! ready, no ALIVE, a dead ALIVE status, a command with no answer, a
//! firmware error mid-sequence. What only silicon can show (that the ROM
//! accepts this memory, that the firmware answers like this) is left to a
//! boot; the order, the bounds and the parsing are proven here.

use std::rc::Rc;

use crate::gen3::bringup::{boot, BootError};
use crate::gen3::cmds::*;
use crate::gen3::dev::{Dev, WaitError};
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::plan::{control_len, RB_REGION};
use crate::gen3::region::Region;
use crate::gen3::select::{transport, Transport};
use crate::gen3::start::StartError;
use crate::gen3::sweep::{Sweep, Tick};
use crate::gen3::ucode::Ucode;
use crate::gen3::up::{up, UpError, SCAN_IF_ADDR};
use crate::gen3_model::{reply, Mem, Model, Out, Polls, Seen};

static SO_GF: &[u8] = include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

struct Rig {
    model: Model,
    fw: Vec<Rc<Mem>>,
}

fn rig() -> Rig {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let ctrl = Mem::new(control_len(ucode.iml.len()).unwrap(), 0x1000_0000);
    let rbs = Mem::new(RB_REGION, 0x2000_0000);
    let layout = classify(SO_GF);
    let sizes = firmware_region_sizes(&layout).unwrap();
    let fw = sizes.iter().enumerate().map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000)).collect();
    Rig { model: Model::new(ctrl, rbs), fw }
}

fn board() -> Board {
    Board { hw_rev: 0x0000_0370, imr_enabled: false, rf_id: 0x0010_D000, pnvm: None }
}

fn so() -> Transport {
    transport(0x51F0).unwrap()
}

fn run_boot(r: &Rig, c: &mut Polls) -> Result<crate::gen3::alive::Alive, BootError> {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    boot(&mut dev, c, &mem, &layout, &ucode, &board())
}

fn polls() -> Polls {
    Polls { per_wait: 64, delays_us: 0 }
}

#[test]
fn the_firmware_boots_to_alive_with_its_memory_where_the_context_info_says() {
    let r = rig();
    let mut c = polls();
    let alive = run_boot(&r, &mut c).expect("alive");
    assert!(alive.ok());
    // The context info names the control region's structures by address.
    let ctrl = &r.model.ctrl;
    let mut ci = [0u8; 104];
    ctrl.read(0, &mut ci);
    let rd64 = |o: usize| u64::from_le_bytes(ci[o..o + 8].try_into().unwrap());
    assert_eq!(rd64(8), ctrl.dev() + 0x2000, "peripheral info page");
    assert_eq!(rd64(88), ctrl.dev() + 0x1000, "peripheral scratch");
    assert_eq!(u32::from_le_bytes(ci[96..100].try_into().unwrap()), 1660, "scratch size without FSEQ");
    assert_eq!(u16::from_le_bytes([ci[68], ci[69]]), 4, "128-entry command ring");
    assert_eq!(u16::from_le_bytes([ci[70], ci[71]]), 9, "512-entry completion ring");
    // The kick went to the control region; the CPU was started through the
    // UMAC window with the 24-bit address intact.
    assert_eq!(r.model.reg(0x118) as u64 | (r.model.reg(0x11C) as u64) << 32, ctrl.dev());
    assert!(r.model.s.borrow().prph.contains(&(0xD0_5C44, 1)), "UREG_CPU_INIT_RUN at its UMAC address");
    // Every section is in its block: the first LMAC section's bytes are there.
    let layout = classify(SO_GF);
    let first = layout.lmac[0].data;
    assert_eq!(&r.fw[0].bytes.borrow()[..first.len()], first);
}

#[test]
fn a_part_with_a_sku_id_rings_the_platform_nvm_doorbell_and_waits() {
    let r = rig();
    r.model.s.borrow_mut().alive_sku = [1, 2, 3];
    let mut c = polls();
    let alive = run_boot(&r, &mut c).expect("alive and the PNVM step done");
    assert_eq!(alive.sku_id, [1, 2, 3]);
    assert!(r.model.s.borrow().prph.contains(&(0xD0_5C04, 1 << 20)), "the PNVM doorbell");
}

#[test]
fn a_nic_that_never_reports_ready_stops_the_boot() {
    let r = rig();
    r.model.s.borrow_mut().ready_never = true;
    let mut c = polls();
    assert_eq!(run_boot(&r, &mut c), Err(BootError::Start(StartError::NotReady)));
}

#[test]
fn no_alive_cause_is_a_bounded_failure() {
    let r = rig();
    r.model.s.borrow_mut().no_alive = true;
    let mut c = polls();
    assert_eq!(run_boot(&r, &mut c), Err(BootError::NoAliveCause));
}

#[test]
fn a_dead_alive_status_is_refused() {
    let r = rig();
    r.model.s.borrow_mut().alive_status = 0xDEAD;
    let mut c = polls();
    assert_eq!(run_boot(&r, &mut c), Err(BootError::NotAlive(0xDEAD)));
}

/// What the modeled firmware answers to each command it is sent.
type Answers = Box<dyn FnMut(&Seen) -> Vec<Out>>;

// The answers a unified-image firmware gives to the bring-up commands.
fn firmware_answers(ax_mcc: bool) -> Answers {
    Box::new(move |s: &Seen| match (s.group, s.cmd) {
        (0x0C, 0x00) => vec![reply(s, vec![]), Out { cmd: 0x04, group: 1, seq: 0x8000, payload: vec![] }],
        (0x0C, 0x02) => {
            let mut p = vec![0u8; 468];
            p[12] = 3;
            p[16] = 3;
            p[20] = 1; // LAR
            reply_with(s, p)
        }
        (0x01, 0xC8) => {
            let mut p = vec![0u8; if ax_mcc { 20 } else { 16 }];
            p[4..6].copy_from_slice(&0x5553u16.to_le_bytes());
            reply_with(s, p)
        }
        (0x01, 0x0D) => {
            let mut beacon = vec![0u8; 64];
            beacon[0..2].copy_from_slice(&(24u16 + 12 + 6 + 4).to_le_bytes());
            beacon[2] = 0x20; // MIC/CRC length 2 << 1: four bytes at the end
            beacon[12] = 0x03; // CRC and overrun OK
            beacon[42] = 6; // channel
            beacon.extend_from_slice(&[0x80, 0, 0, 0]);
            beacon.extend_from_slice(&[0xFF; 6]);
            beacon.extend_from_slice(&[0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
            beacon.extend_from_slice(&[0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
            beacon.extend_from_slice(&[0, 0]);
            beacon.extend_from_slice(&[0; 8]);
            beacon.extend_from_slice(&[0x64, 0, 0x11, 0x04]);
            beacon.extend_from_slice(&[0, 4]);
            beacon.extend_from_slice(b"Home");
            beacon.extend_from_slice(&[0xC1, 0xC2, 0xC3, 0xC4]);
            vec![
                reply(s, vec![]),
                Out { cmd: 0xC1, group: 0, seq: 0x8000, payload: beacon },
                Out { cmd: 0x0F, group: 0, seq: 0x8000, payload: vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
            ]
        }
        _ => vec![reply(s, vec![])],
    })
}

fn reply_with(s: &Seen, p: Vec<u8>) -> Vec<Out> {
    vec![reply(s, p)]
}

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

/// The interface takes the address bring-up drew for this boot, in both the
/// MAC and the link context.
#[test]
fn the_interface_is_created_at_the_station_address_it_is_given() {
    let r = rig();
    let ucode = Ucode::parse(SO_GF).unwrap();
    r.model.s.borrow_mut().responder = Some(firmware_answers(ucode.capa(CAPA_MCC_UPDATE_11AX_SUPPORT)));
    let mut c = polls();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &board()).expect("alive");
    let drawn = [0x5E, 0x11, 0x22, 0x33, 0x44, 0x55];
    up(&mut dev, &mut c, &ucode, &so(), drawn).expect("up");
    let cmds = r.model.s.borrow().commands.clone();
    let mac = cmds.iter().find(|s| (s.group, s.cmd) == (MAC_CONF_GROUP, MAC_CONFIG_CMD)).unwrap();
    let link = cmds.iter().find(|s| (s.group, s.cmd) == (MAC_CONF_GROUP, LINK_CONFIG_CMD)).unwrap();
    assert_eq!(&mac.payload[12..18], &drawn);
    assert_eq!(&link.payload[16..22], &drawn);
}

#[test]
fn a_command_without_an_answer_names_itself() {
    let r = rig();
    r.model.s.borrow_mut().responder = Some(Box::new(|s: &Seen| {
        if (s.group, s.cmd) == (0x0C, 0x02) {
            vec![] // NVM_GET_INFO never answered
        } else if (s.group, s.cmd) == (0x0C, 0x00) {
            vec![reply(s, vec![]), Out { cmd: 0x04, group: 0, seq: 0x8000, payload: vec![] }]
        } else {
            vec![reply(s, vec![])]
        }
    }));
    let mut c = polls();
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &board()).unwrap();
    assert_eq!(
        up(&mut dev, &mut c, &ucode, &so(), SCAN_IF_ADDR),
        Err(UpError::Command(REGULATORY_AND_NVM_GROUP, NVM_GET_INFO, WaitError::TimedOut))
    );
}

#[test]
fn a_firmware_error_mid_sequence_is_reported_as_such() {
    let r = rig();
    r.model.s.borrow_mut().fw_error_after = Some(1);
    let mut c = polls();
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &board()).unwrap();
    assert_eq!(
        up(&mut dev, &mut c, &ucode, &so(), SCAN_IF_ADDR),
        Err(UpError::Command(REGULATORY_AND_NVM_GROUP, NVM_ACCESS_COMPLETE, WaitError::FirmwareError))
    );
}
