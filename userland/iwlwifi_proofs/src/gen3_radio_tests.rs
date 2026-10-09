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

//! The serving-loop side of the gen3 radio, against the modeled device: the
//! background sweep (completes, ignores another scan's completion, gives up
//! a sweep past its budget, stops on a firmware error, refuses an empty
//! channel list), what it hears into the shared scan list, the device masked
//! once ALIVE is in and stopped after a failed boot, the secure-boot status
//! read through the UMAC window, the bounds every grant access is checked
//! against, how each failure is reported, the control family's replies, and
//! which legacy ops are refused once the radio owns the card.

use nonos_wifi_core::scan_list::{ScanResults, FLAG_WPA2};

use crate::control::{answer, Heard, View, REPLY_MAX};
use crate::gen3::bringup::{boot, BootError};
use crate::gen3::dev::{Dev, WaitError};
use crate::gen3::dram_map::classify;
use crate::gen3::heard::hear;
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::outcome::*;
use crate::gen3::plan::{control_len, RB_REGION};
use crate::gen3::prph::read_umac;
use crate::gen3::region::span;
use crate::gen3::regs::*;
use crate::gen3::rx_frame::RxFrame;
use crate::gen3::select::Refusal;
use crate::gen3::start::{stop_device, StartError};
use crate::gen3::sweep::{budget_ms, ScanError, Sweep, Tick};
use crate::gen3::ucode::Ucode;
use crate::gen3::up::UpError;
use crate::gen3_model::{reply, Mem, Model, Out, Polls, Seen};
use crate::guard::drives_card;
use crate::protocol::*;

static SO_GF: &[u8] = include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

struct Rig {
    model: Model,
    fw: Vec<std::rc::Rc<Mem>>,
}

fn rig() -> Rig {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let ctrl = Mem::new(control_len(ucode.iml.len()).unwrap(), 0x1000_0000);
    let rbs = Mem::new(RB_REGION, 0x2000_0000);
    let sizes = firmware_region_sizes(&classify(SO_GF)).unwrap();
    let fw = sizes.iter().enumerate().map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000)).collect();
    Rig { model: Model::new(ctrl, rbs), fw }
}

fn polls() -> Polls {
    Polls { per_wait: 64, delays_us: 0 }
}

// Boot the rig to ALIVE and run `f` with the live transport.
fn booted(r: &Rig, f: impl FnOnce(&mut Dev<'_, Model, Mem>, &mut Polls)) {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    let mut c = polls();
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &Board { hw_rev: 0x370, imr_enabled: false, rf_id: 0x10D000, pnvm: None }).expect("alive");
    f(&mut dev, &mut c);
}

fn completion(uid: u32, status: u8) -> Out {
    let mut p = vec![0u8; 16];
    p[0..4].copy_from_slice(&uid.to_le_bytes());
    p[6] = status;
    Out { cmd: 0x0F, group: 0, seq: 0x8000, payload: p }
}

// A beacon for `ssid` from `bssid`, with an RSN element offering PSK.
fn beacon(bssid: [u8; 6], ssid: &[u8]) -> Vec<u8> {
    let mut f = vec![0x80, 0, 0, 0];
    f.extend_from_slice(&[0xFF; 6]);
    f.extend_from_slice(&bssid);
    f.extend_from_slice(&bssid);
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&[0; 8]);
    f.extend_from_slice(&[0x64, 0, 0x11, 0x04]);
    f.push(0);
    f.push(ssid.len() as u8);
    f.extend_from_slice(ssid);
    f.extend_from_slice(&[48, 20, 1, 0, 0x00, 0x0F, 0xAC, 4, 1, 0, 0x00, 0x0F, 0xAC, 4, 1, 0, 0x00, 0x0F, 0xAC, 2, 0, 0]);
    f
}

#[test]
fn a_sweep_completes_on_its_own_uid_and_ignores_another() {
    let r = rig();
    r.model.s.borrow_mut().responder = Some(Box::new(|s: &Seen| {
        if (s.group, s.cmd) == (1, 0x0D) {
            vec![reply(s, vec![]), completion(7, 1)]
        } else {
            vec![reply(s, vec![])]
        }
    }));
    booted(&r, |dev, c| {
        let mut sweep = Sweep::new();
        assert_eq!(sweep.start(dev, c, &[1, 6, 11], 0, &mut |_| {}), Ok(Tick::Running));
        assert_eq!(sweep.pump(dev, 5, &mut |_| {}), Tick::Running, "uid 7 is not this driver's scan");
        r.model.queue(completion(0, 1));
        assert_eq!(sweep.pump(dev, 6, &mut |_| {}), Tick::Completed(1));
        assert!(!sweep.running());
        assert_eq!(sweep.pump(dev, 7, &mut |_| {}), Tick::Idle, "nothing in flight");
    });
}

#[test]
fn a_sweep_past_its_budget_is_given_up() {
    let r = rig();
    booted(&r, |dev, c| {
        let mut sweep = Sweep::new();
        let channels = [1u8, 6, 11];
        let budget = u64::from(budget_ms(channels.len()));
        assert_eq!(budget, 3 * 150 + 2000);
        assert_eq!(sweep.start(dev, c, &channels, 1000, &mut |_| {}), Ok(Tick::Running));
        assert_eq!(sweep.pump(dev, 1000 + budget, &mut |_| {}), Tick::Running, "still within budget");
        assert_eq!(sweep.pump(dev, 1001 + budget, &mut |_| {}), Tick::Stalled);
        assert_eq!((sweep.stalled, sweep.completed, sweep.running()), (1, 0, false));
    });
}

#[test]
fn a_firmware_error_ends_the_sweep() {
    let r = rig();
    booted(&r, |dev, c| {
        let mut sweep = Sweep::new();
        assert_eq!(sweep.start(dev, c, &[1], 0, &mut |_| {}), Ok(Tick::Running));
        r.model.s.borrow_mut().regs.insert(CSR_INT, INT_BIT_SW_ERR);
        assert_eq!(sweep.pump(dev, 1, &mut |_| {}), Tick::Failed);
        assert!(!sweep.running());
        assert_eq!(sweep.pump(dev, 2, &mut |_| {}), Tick::Failed, "the cause stays visible when idle");
    });
}

#[test]
fn a_sweep_needs_channels_and_an_answer() {
    let r = rig();
    booted(&r, |dev, c| {
        let mut sweep = Sweep::new();
        assert_eq!(sweep.start(dev, c, &[], 0, &mut |_| {}), Err(ScanError::NoChannels));
        assert_eq!(sweep.start(dev, c, &[1; 68], 0, &mut |_| {}), Err(ScanError::NoChannels), "over 67");
        r.model.s.borrow_mut().responder = Some(Box::new(|_: &Seen| vec![]));
        assert_eq!(sweep.start(dev, c, &[1], 0, &mut |_| {}), Err(ScanError::Request(WaitError::TimedOut)));
        assert!(!sweep.running());
    });
}

#[test]
fn beacons_heard_reach_the_scan_list_with_signal_and_security() {
    let mut list = ScanResults::new();
    let ap = [0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];
    let f = RxFrame { frame: beacon(ap, b"Home"), channel: 6, signal: -60 };
    assert!(hear(&mut list, &f));
    let mut out = [0u8; 64];
    let n = list.encode(&mut out);
    assert_eq!(&out[..n], &[1, 80, 1 | FLAG_WPA2, 4, b'H', b'o', b'm', b'e']);
    // A probe request, and a beacon cut short, are not listed.
    let mut probe = beacon(ap, b"Other");
    probe[0] = 0x40;
    assert!(!hear(&mut list, &RxFrame { frame: probe, channel: 6, signal: -40 }));
    let short = beacon(ap, b"Home")[..30].to_vec();
    assert!(!hear(&mut list, &RxFrame { frame: short, channel: 6, signal: -40 }));
    assert_eq!(list.count(), 1);
}

#[test]
fn the_device_stops_signalling_interrupts_once_alive() {
    let r = rig();
    booted(&r, |_, _| {});
    assert_eq!(r.model.reg(CSR_INT_MASK), 0);
    assert_eq!(r.model.reg(CSR_MSIX_HW_INT_MASK_AD), 0xFFFF_FFFF);
    assert_eq!(r.model.reg(CSR_MSIX_FH_INT_MASK_AD), 0xFFFF_FFFF);
}

#[test]
fn a_failed_boot_leaves_the_device_stopped() {
    let r = rig();
    r.model.s.borrow_mut().no_alive = true;
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    let mut c = polls();
    let board = Board { hw_rev: 0x370, imr_enabled: false, rf_id: 0x10D000, pnvm: None };
    assert_eq!(boot(&mut dev, &mut c, &mem, &layout, &ucode, &board), Err(BootError::NoAliveCause));
    let before = c.delays_us;
    assert!(stop_device(&r.model, &mut c), "the bus master reported stopped");
    let reset = r.model.reg(CSR_RESET);
    assert_eq!(reset & (RESET_STOP_MASTER | RESET_SW_RESET), RESET_STOP_MASTER | RESET_SW_RESET);
    assert_eq!(r.model.reg(CSR_INT_MASK), 0);
    assert_eq!(r.model.reg(CSR_MSIX_HW_INT_MASK_AD), 0xFFFF_FFFF);
    assert!(c.delays_us - before >= 6000, "the reset is given its settle time");
}

#[test]
fn secure_boot_status_is_read_through_the_umac_window() {
    let r = rig();
    r.model.s.borrow_mut().regs.insert(HBUS_TARG_PRPH_RDAT, 0x0000_0101);
    let mut c = polls();
    assert_eq!(read_umac(&r.model, &mut c, UMAG_SB_CPU_1_STATUS), Some(0x101));
    assert_eq!(r.model.reg(HBUS_TARG_PRPH_RADDR), 0x00D0_38C0 | 3 << 24, "24-bit address, byte enables");
}

#[test]
fn every_grant_access_is_bounded() {
    assert_eq!(span(4096, 0, 4096), Some(0..4096));
    assert_eq!(span(4096, 4095, 1), Some(4095..4096));
    assert_eq!(span(4096, 4095, 2), None, "one past the end");
    assert_eq!(span(4096, 4097, 0), None, "an empty read past the end");
    assert_eq!(span(4096, usize::MAX, 2), None, "the end overflows");
    assert_eq!(span(0, 0, 0), Some(0..0));
}

#[test]
fn each_failure_names_its_stage_step_and_detail() {
    let cases = [
        (Failure::Window, STAGE_DEAD_MMIO, STEP_WINDOW, 0),
        (Failure::DeadMmio, STAGE_DEAD_MMIO, STEP_DEAD_MMIO, 0),
        (Failure::Start(StartError::NotReady), STAGE_POWER_FAILED, STEP_START, 1),
        (Failure::Refused(Refusal::NotSoDevice(0x2725)), STAGE_NO_AIR_PATH, STEP_REFUSED, 0x1_2725),
        (Failure::Refused(Refusal::RfNotBundled(0x105)), STAGE_NO_AIR_PATH, STEP_REFUSED, 0x3_0105),
        (Failure::Image, STAGE_FIRMWARE_FAILED, STEP_IMAGE, 0),
        (Failure::NoDma, STAGE_NO_DMA, STEP_NO_DMA, 0),
        (Failure::Boot(BootError::Start(StartError::ClockNotReady)), STAGE_POWER_FAILED, STEP_BOOT, 0x12),
        (Failure::Boot(BootError::NoNicAccess), STAGE_POWER_FAILED, STEP_BOOT, 0x70),
        (Failure::Boot(BootError::NoAliveCause), STAGE_FIRMWARE_FAILED, STEP_BOOT, 0x30),
        (Failure::Boot(BootError::NotAlive(0xDEAD)), STAGE_FIRMWARE_FAILED, STEP_BOOT, 0x5_DEAD),
        (Failure::Boot(BootError::Pnvm(WaitError::TimedOut)), STAGE_FIRMWARE_FAILED, STEP_BOOT, 0x61),
        (Failure::Up(UpError::Command(0x0C, 0x02, WaitError::TimedOut)), STAGE_FIRMWARE_FAILED, STEP_UP, 0x010C_0201),
        (Failure::Up(UpError::BadNvm), STAGE_FIRMWARE_FAILED, STEP_UP, 0x0300_0000),
        (Failure::Lost, STAGE_FIRMWARE_FAILED, STEP_LOST, 0),
    ];
    for (f, stage, step, detail) in cases {
        assert_eq!((f.stage(), f.step(), f.detail()), (stage, step, detail), "{f:?}");
        assert!(!f.text().is_empty());
    }
    // The stage codes are the client's: Ready 0, PowerFailed 2, DeadMmio 3,
    // FirmwareFailed 4, NoDma 5, NoAirPath 8.
    assert_eq!((STAGE_READY, STAGE_POWER_FAILED, STAGE_DEAD_MMIO), (0, 2, 3));
    assert_eq!((STAGE_FIRMWARE_FAILED, STAGE_NO_DMA, STAGE_NO_AIR_PATH), (4, 5, 8));
}

fn request(op: u16, rid: u32) -> Vec<u8> {
    let mut r = 0x5749_4649u32.to_le_bytes().to_vec();
    r.extend_from_slice(&op.to_le_bytes());
    r.extend_from_slice(&rid.to_le_bytes());
    r
}

fn word(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

#[test]
fn a_radio_that_is_down_reports_how_far_it_got_and_refuses_the_rest() {
    let f = Failure::Boot(BootError::NoAliveCause);
    let view = View { stage: f.stage(), step: f.step(), detail: f.detail(), hw_rev: 0x370, rf_id: 0x10D000, heard: None };
    let mut out = vec![0u8; REPLY_MAX];
    let n = answer(&request(4, 77), &view, &mut out).unwrap();
    assert_eq!(n, 10 + 30);
    assert_eq!(&out[..10], &request(4, 77)[..], "the header is echoed");
    assert_eq!((out[10], out[11]), (STAGE_FIRMWARE_FAILED, STEP_BOOT));
    assert_eq!((word(&out, 12), word(&out, 16), word(&out, 20)), (0x30, 0x370, 0x10D000));
    for op in [1u16, 2, 3, 5, 99] {
        let n = answer(&request(op, 1), &view, &mut out).unwrap();
        assert_eq!((n, word(&out, 10) as i32), (14, -38), "op {op}");
    }
}

#[test]
fn a_scanning_radio_reports_ready_and_answers_a_scan_from_what_it_heard() {
    let mut list = ScanResults::new();
    list.heard([2, 0, 0, 0, 0, 1], b"Home", 1 | FLAG_WPA2, 80);
    let heard = || Some(Heard { list: &list, sweeps: 3, stalls: 1, frames: 40, beacons: 25 });
    let view = View { stage: STAGE_READY, step: 0, detail: 0, hw_rev: 0x370, rf_id: 0x10D000, heard: heard() };
    let mut out = vec![0u8; REPLY_MAX];
    let n = answer(&request(4, 1), &view, &mut out).unwrap();
    assert_eq!(out[10], 0, "Ready");
    assert_eq!((word(&out, 24), word(&out, 28), word(&out, 32), word(&out, 36)), (3, 1, 40, 25));
    assert_eq!(n, 40);
    let n = answer(&request(3, 2), &view, &mut out).unwrap();
    assert_eq!((word(&out, 10), word(&out, 14), word(&out, 18)), (3, 40, 25), "sweeps, frames, beacons");
    assert_eq!(&out[22..n], &[1, 80, 1 | FLAG_WPA2, 4, b'H', b'o', b'm', b'e']);
    // A join that reaches `answer` (the radio cannot run one) is refused,
    // and says so at once.
    let n = answer(&request(1, 3), &view, &mut out).unwrap();
    assert_eq!((n, word(&out, 10) as i32), (14, -38));
}

#[test]
fn a_full_scan_list_fits_the_reply() {
    let mut list = ScanResults::new();
    for i in 0..20u8 {
        list.heard([2, 0, 0, 0, 1, i], &[b'n'; 32], 1, i);
    }
    let view = View {
        stage: 0,
        step: 0,
        detail: 0,
        hw_rev: 0,
        rf_id: 0,
        heard: Some(Heard { list: &list, sweeps: 1, stalls: 0, frames: 0, beacons: 0 }),
    };
    let mut out = vec![0u8; REPLY_MAX];
    let n = answer(&request(3, 1), &view, &mut out).unwrap();
    assert_eq!(n, REPLY_MAX, "sixteen networks at the longest SSID");
    assert_eq!(out[22], 16);
}

#[test]
fn what_is_not_a_control_request_goes_on_to_the_driver_protocol() {
    let view = View { stage: 0, step: 0, detail: 0, hw_rev: 0, rf_id: 0, heard: None };
    let mut out = vec![0u8; REPLY_MAX];
    let mut niwf = 0x4E49_5746u32.to_le_bytes().to_vec();
    niwf.extend_from_slice(&[0; 16]);
    assert_eq!(answer(&niwf, &view, &mut out), None, "the driver's own tag");
    assert_eq!(answer(&request(4, 1)[..9], &view, &mut out), None, "shorter than the header");
    let mut small = vec![0u8; REPLY_MAX - 1];
    assert_eq!(answer(&request(4, 1), &view, &mut small), None, "a reply buffer too small for a scan");
}

#[test]
fn only_the_legacy_ops_that_write_the_card_are_refused_once_the_radio_owns_it() {
    for op in [OP_FIRMWARE_LOAD, OP_ALIVE_WAIT, OP_HCMD_ISSUE] {
        assert!(drives_card(op), "{op:#x}");
    }
    for op in [
        OP_HEALTHCHECK,
        OP_DEVICE_INFO,
        OP_FIRMWARE_INFO,
        OP_RF_STATE,
        OP_DMA_STATE,
        OP_FIRMWARE_STAGE,
        OP_RX_POLL,
        OP_MGMT_BUILD,
        OP_BEACON_PARSE,
        OP_WPA_PTK,
        OP_CCMP,
        OP_WIFI_TX,
        OP_WIFI_RX,
    ] {
        assert!(!drives_card(op), "{op:#x}");
    }
}

/// The Wi-Fi client sends a join only to a driver its table says joins. This
/// driver runs a join whenever its radio can (the request goes to the radio,
/// not to `answer`), so the table lists it as joining; a radio that cannot
/// still answers with the code the client gives a driver that does not join.
#[test]
fn the_client_sends_joins_to_this_driver_and_it_runs_them() {
    use crate::control::{route, Route};
    use crate::wifi_services::{CANNOT_JOIN, SERVICES};
    const OP_CONNECT: u16 = 1;
    let list = ScanResults::new();
    let mut join = request(OP_CONNECT, 9);
    join.extend_from_slice(&[4, b'h', b'o', b'm', b'e', 8]);
    join.extend_from_slice(b"passw0rd");
    join.push(0);
    assert_eq!(route(&join, true), Route::Join(OP_CONNECT), "a radio that can join runs it");
    for heard in [None, Some(Heard { list: &list, sweeps: 1, stalls: 0, frames: 0, beacons: 0 })] {
        let view = View { stage: STAGE_READY, step: 0, detail: 0, hw_rev: 0, rf_id: 0, heard };
        assert_eq!(route(&join, false), Route::Control);
        let mut out = vec![0u8; REPLY_MAX];
        let n = answer(&join, &view, &mut out).expect("a control request is answered");
        assert_eq!(n, 10 + 4);
        assert_eq!(word(&out, 10) as i32, CANNOT_JOIN, "the join was answered as something else");
    }
    let me = SERVICES.iter().find(|s| s.name == b"driver.iwlwifi0").expect("listed");
    assert!(me.joins, "the client would answer for this driver instead of asking it");
    let realtek = SERVICES.iter().find(|s| s.name == b"driver.rtl8821ce0").expect("listed");
    assert!(realtek.joins);
    assert_eq!(SERVICES[0].name, b"driver.rtl8821ce0", "the RTL8821CE is still asked first");
}

