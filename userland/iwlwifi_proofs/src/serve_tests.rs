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

//! The serving loop's side of joining: which requests go where (net_core's
//! link protocol to the link, the join ops to the radio only when it can
//! join, the rest as before), the connect body read with every length
//! checked, every way a join ends mapped to the code the panels name, the
//! connect and link replies read back by the client's own parsers, and the
//! hunt for the network's beacon against the modeled firmware (found and
//! the sweep stopped, not heard, a background sweep stopped first).

use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::SelectError;
use nonos_wifi_core::sae::SaeFailure;
use nonos_wifi_core::wpa::supplicant::Failure;

use crate::client_join_wire::{parse_connect_reply, parse_link};
use crate::control::{answer, route, Route, View, REPLY_MAX};
use crate::gen3::dev::WaitError;
use crate::gen3::join::bss::SetupError;
use crate::gen3::join::exchange::Progress;
use crate::gen3::join::fw::CommandFailed;
use crate::gen3::join::hunt::{hunt, HuntEnd};
use crate::gen3::join::keys::KeyError;
use crate::gen3::join::run::JoinEnd;
use crate::gen3::sweep::{Sweep, Tick};
use crate::gen3_model::{reply, Out, Seen};
use crate::join_rig::{beacon, rig, Security, Station, Switches, AP, RSNE_PSK};
use crate::join_wire::*;

fn control(op: u16) -> Vec<u8> {
    let mut v = 0x5749_4649u32.to_le_bytes().to_vec();
    v.extend_from_slice(&op.to_le_bytes());
    v.extend_from_slice(&7u32.to_le_bytes());
    v
}

#[test]
fn requests_go_to_the_link_the_radio_or_as_before() {
    let mut nnet = 0x4E4E_4554u32.to_le_bytes().to_vec();
    nnet.extend_from_slice(&[0; 16]);
    for can in [false, true] {
        assert_eq!(route(&nnet, can), Route::Link, "net_core's link protocol");
        assert_eq!(route(&control(3), can), Route::Control, "scan");
        assert_eq!(route(&control(4), can), Route::Control, "status");
        assert_eq!(route(&control(4)[..9], can), Route::Driver, "shorter than the header");
        assert_eq!(route(&0x4E49_5746u32.to_le_bytes(), can), Route::Driver, "this driver's own");
        assert_eq!(route(&[1, 2], can), Route::Driver);
    }
    for op in [1u16, 2, 5] {
        assert_eq!(route(&control(op), true), Route::Join(op));
        assert_eq!(route(&control(op), false), Route::Control, "a radio that cannot join answers as before");
    }
    // Answered as before: -38, and nothing of the body is kept.
    let view = View { stage: 0, step: 0, detail: 0, hw_rev: 0, rf_id: 0, heard: None };
    let mut out = vec![0u8; REPLY_MAX];
    let n = answer(&control(1), &view, &mut out).unwrap();
    assert_eq!((n, i32::from_le_bytes(out[10..14].try_into().unwrap())), (14, -38));
}

#[test]
fn a_connect_body_is_read_with_every_length_checked() {
    let mut b = vec![4];
    b.extend_from_slice(b"home");
    b.push(8);
    b.extend_from_slice(b"passw0rd");
    let r = parse_connect(&b).unwrap();
    assert_eq!((r.ssid, r.pass, r.wpa3_only, r.hidden), (&b"home"[..], &b"passw0rd"[..], false, false));
    b.push(0b11);
    let r = parse_connect(&b).unwrap();
    assert!(r.wpa3_only && r.hidden);
    assert!(parse_connect(&b[..5]).is_none(), "no passphrase length");
    assert!(parse_connect(&b[..12]).is_none(), "the passphrase runs past the body");
    assert!(parse_connect(&[0, 0]).is_none(), "an empty SSID");
    let mut long = vec![33];
    long.extend_from_slice(&[b'a'; 33]);
    long.push(0);
    assert!(parse_connect(&long).is_none(), "an SSID longer than 32");
    assert!(parse_connect(&[]).is_none());
}

#[test]
fn every_end_maps_to_the_code_the_panels_name() {
    let fail = CommandFailed { group: 1, cmd: 8, why: WaitError::TimedOut };
    let cases = [
        (JoinEnd::NotTheNetwork, (CODE_NOT_FOUND, 0)),
        (JoinEnd::TimedOut, (CODE_TIMED_OUT, 0)),
        (JoinEnd::Setup(SetupError::BadQueueReply), (CODE_BAD_REQUEST, 0)),
        (JoinEnd::Firmware(Some(fail)), (CODE_BAD_REQUEST, 0)),
        (JoinEnd::Firmware(None), (CODE_BAD_REQUEST, 0)),
        (JoinEnd::Keys(KeyError::Pairwise(fail)), (CODE_PAIRWISE_KEY, 0)),
        (JoinEnd::Keys(KeyError::Group(None)), (CODE_GROUP_KEY, 0)),
        (JoinEnd::Refused(MlmeFailure::AuthRejected(17)), (CODE_REFUSED, 17)),
        (JoinEnd::Refused(MlmeFailure::AssocRejected(18)), (CODE_REFUSED, 18)),
        (JoinEnd::Refused(MlmeFailure::Left(15)), (CODE_TIMED_OUT, 15)),
        (JoinEnd::Refused(MlmeFailure::Select(SelectError::Downgrade)), (CODE_DOWNGRADE, 0)),
        (JoinEnd::Refused(MlmeFailure::OpenNetwork), (CODE_UNSUPPORTED, 0)),
        (JoinEnd::Refused(MlmeFailure::NeedsHt), (CODE_UNSUPPORTED, 0)),
        (JoinEnd::Refused(MlmeFailure::BadPassphrase), (CODE_BAD_PASSPHRASE, 0)),
        (JoinEnd::Refused(MlmeFailure::NoEntropy), (CODE_NO_ENTROPY, 0)),
        (JoinEnd::Refused(MlmeFailure::Sae(SaeFailure::BadConfirm)), (CODE_WRONG_PASSWORD, 0)),
        (JoinEnd::Refused(MlmeFailure::Sae(SaeFailure::Rejected(77))), (CODE_REFUSED, 77)),
        (JoinEnd::Refused(MlmeFailure::Handshake(Failure::IeMismatch)), (CODE_IE_MISMATCH, 0)),
    ];
    for (end, want) in cases {
        assert_eq!(end_code(end), want, "{end:?}");
    }
}

#[test]
fn the_client_reads_the_connect_and_link_replies_as_written() {
    let mut out = vec![0u8; 64];
    let r = ConnectResult {
        code: CODE_REFUSED,
        progress: Progress { sent: 3, recv: 4, data: 5, eapol: 6, probe: 7, deauth: 1, to_us: 2, refused: 0, state: 2 },
        state: 2,
        akm: 8,
        ap_code: 17,
    };
    let n = encode_connect(&mut out, &r).unwrap();
    assert_eq!(n, 10 + 36);
    let c = parse_connect_reply(&out[10..n]).unwrap();
    assert_eq!((c.code, c.sent, c.recv, c.data, c.eapol, c.probe, c.deauth, c.to_us), (-5, 3, 4, 5, 6, 7, 1, 2));
    assert_eq!((c.state, c.akm, c.ap_code), (2, 8, 17));
    assert!(encode_connect(&mut out[..45], &r).is_none(), "too small a buffer is refused");

    let info = LinkInfo { bssid: AP, ssid: b"home", akm: 8 };
    let n = encode_link(&mut out, Some(&info)).unwrap();
    let l = parse_link(&out[10..n]).unwrap();
    assert!(l.associated);
    assert_eq!((l.bssid, l.ssid(), l.akm), (AP, &b"home"[..], 8));
    assert!(l.joined_with_sae(b"home"));
    let n = encode_link(&mut out, None).unwrap();
    assert!(!parse_link(&out[10..n]).unwrap().associated);
    let n = encode_code(&mut out, 0).unwrap();
    assert_eq!((n, &out[10..14]), (14, &[0u8; 4][..]));
}

// A beacon or probe response notification heard on `channel`.
fn heard(frame: &[u8], channel: u8) -> Out {
    let mut p = vec![0u8; 64];
    p[0..2].copy_from_slice(&(frame.len() as u16).to_le_bytes());
    p[12] = 0x3;
    p[40] = 50;
    p[42] = channel;
    p.extend_from_slice(frame);
    Out { cmd: 0xC1, group: 0, seq: 0x8000, payload: p }
}

fn completion(status: u8) -> Out {
    let mut p = vec![0u8; 16];
    p[6] = status;
    Out { cmd: 0x0F, group: 0, seq: 0x8000, payload: p }
}

fn other_network() -> Vec<u8> {
    let mut b = beacon(&[&RSNE_PSK]);
    b[38] = b'H';
    b[10] ^= 0x10;
    b[16] ^= 0x10;
    b
}

#[test]
fn the_hunt_stops_the_sweep_once_the_network_is_heard() {
    let r = rig(Security::Wpa2, Switches::default());
    let target = beacon(&[&RSNE_PSK]);
    let t = target.clone();
    r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd) {
        // The scan request: another network on 1, then the target on 6.
        (0x1, 0x0D) => vec![reply(c, vec![]), heard(&other_network(), 1), heard(&t, 6)],
        // The abort: the scan completes as aborted.
        (0x1, 0x0E) => vec![reply(c, vec![]), completion(2)],
        _ => vec![reply(c, vec![])],
    }));
    let mut st = Station::new();
    let mut seen = 0;
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        let mut t = 0u64;
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[1, 6, 11], b"home", &mut || { t += 1; t }, &mut |_| seen += 1);
        assert_eq!(got, Ok((target.clone(), 6)));
        assert!(!sweep.running(), "the radio is the join's");
        assert_eq!(sweep.completed, 1);
    });
    assert_eq!(seen, 2, "every beacon also went to the scan list");
    let cmds: Vec<(u8, u8)> = r.commands();
    assert_eq!(cmds, vec![(0x1, 0x0D), (0x1, 0x0E)], "scan, then abort");
    let abort = r.command_payloads(0x1, 0x0E);
    assert_eq!(abort[0], vec![0u8; 8], "the abort names uid 0");
}

#[test]
fn an_abort_completed_before_its_reply_ends_the_sweep() {
    let r = rig(Security::Wpa2, Switches::default());
    let target = beacon(&[&RSNE_PSK]);
    let t = target.clone();
    r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd) {
        (0x1, 0x0D) => vec![reply(c, vec![]), heard(&t, 11)],
        (0x1, 0x0E) => vec![completion(2), reply(c, vec![])],
        _ => vec![reply(c, vec![])],
    }));
    let mut st = Station::new();
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[11], b"home", &mut || 0, &mut |_| {});
        assert_eq!(got, Ok((target.clone(), 11)), "with the channel it was heard on");
        assert!(!sweep.running());
        assert_eq!(sweep.completed, 1);
    });
}

#[test]
fn a_network_not_heard_in_a_whole_sweep_is_not_found() {
    let r = rig(Security::Wpa2, Switches::default());
    r.model.s.borrow_mut().responder = Some(Box::new(|c: &Seen| match (c.group, c.cmd) {
        (0x1, 0x0D) => vec![reply(c, vec![]), heard(&other_network(), 1), completion(1)],
        _ => vec![reply(c, vec![])],
    }));
    let mut st = Station::new();
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[1, 6], b"home", &mut || 0, &mut |_| {});
        assert_eq!(got, Err(HuntEnd::NotHeard(1)));
        assert!(!sweep.running());
    });
    assert_eq!(r.commands(), vec![(0x1, 0x0D)], "nothing to abort");
}

#[test]
fn a_background_sweep_is_stopped_before_the_hunt_and_a_silent_one_is_given_up() {
    let r = rig(Security::Wpa2, Switches::default());
    let first = std::cell::Cell::new(true);
    r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd) {
        (0x1, 0x0D) if first.replace(false) => vec![reply(c, vec![])],
        (0x1, 0x0D) => vec![reply(c, vec![]), completion(1)],
        (0x1, 0x0E) => vec![reply(c, vec![]), completion(2)],
        _ => vec![reply(c, vec![])],
    }));
    let mut st = Station::new();
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        assert_eq!(sweep.start(fw.dev, fw.clock, &[1], 0, &mut |_| {}), Ok(Tick::Running));
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[1], b"home", &mut || 0, &mut |_| {});
        assert_eq!(got, Err(HuntEnd::NotHeard(0)));
    });
    assert_eq!(r.commands(), vec![(0x1, 0x0D), (0x1, 0x0E), (0x1, 0x0D)], "background, abort, the hunt's own");

    // A firmware that never completes the aborted sweep leaves the radio
    // busy: the hunt gives up rather than join under a running scan.
    let r = rig(Security::Wpa2, Switches::default());
    r.model.s.borrow_mut().responder = Some(Box::new(|c: &Seen| vec![reply(c, vec![])]));
    let mut st = Station::new();
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        assert_eq!(sweep.start(fw.dev, fw.clock, &[1], 0, &mut |_| {}), Ok(Tick::Running));
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[1], b"home", &mut || 0, &mut |_| {});
        assert_eq!(got, Err(HuntEnd::Radio));
    });
}
