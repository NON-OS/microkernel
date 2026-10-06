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

//! The join against the modeled firmware and a scripted access point: WPA2
//! and WPA3-SAE (hash-to-element and hunting and pecking) run to an open port
//! with every firmware command in Linux's order and the keys the access point
//! derived in the firmware's table; data crosses both ways through the
//! net_core link protocol, decrypted by the firmware or by the station; every
//! refusal, silence and firmware failure ends the join with its own reason
//! and takes the contexts down exactly as far as they went; nothing but
//! authentication, association and EAPOL in the clear leaves before the
//! port opens; and leaving, the access point's deauthentication and a group
//! rekey each do what they must.

use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::netif::{serve, wire, LinkPort, MAX_RESPONSE};
use nonos_wifi_core::rsn::{JoinPolicy, SelectError};
use nonos_wifi_core::sae::SaeFailure;
use nonos_wifi_core::wpa::akm::Akm;

use crate::ap_sim::{gtk_kde, group1, GTK};
use crate::gen3::join::bss::{SetupError, Stage};
use crate::gen3::join::inbox::{Inbox, Queues, INBOX_FRAMES};
use crate::gen3::join::target::target;
use crate::gen3::packet::Packet;
use crate::gen3::txq::{TxQueue, TXQ_STRIDE};
use crate::gen3::join::exchange::limits::IDLE_TRIES;
use crate::gen3::join::exchange::Progress;
use crate::gen3::join::keys::KeyError;
use crate::gen3::join::link::Port;
use crate::gen3::join::run::{join, leave, JoinEnd, Joined};
use crate::gen3::station::session::Session;
use crate::gen3::tx_cmd::{IWL_TX_FLAGS_CMD_RATE, IWL_TX_FLAGS_ENCRYPT_DIS};
use crate::gen3_model::{reply, Out, Seen};
use crate::join_rig::*;

const PHY: (u8, u8) = (0x1, 0x08);
const RLC: (u8, u8) = (0x5, 0x08);
const LINK: (u8, u8) = (0x3, 0x09);
const STA_CFG: (u8, u8) = (0x3, 0x0A);
const STA_RM: (u8, u8) = (0x3, 0x0C);
const QUEUE: (u8, u8) = (0x5, 0x17);
const SESSION: (u8, u8) = (0x3, 0x05);
const MAC: (u8, u8) = (0x3, 0x08);
const KEY: (u8, u8) = (0x5, 0x18);
const FLUSH: (u8, u8) = (0x1, 0x1E);

/// Up to the open port, in Linux's order.
const JOIN_COMMANDS: [(u8, u8); 15] =
    [PHY, RLC, LINK, LINK, STA_CFG, QUEUE, QUEUE, SESSION, MAC, STA_CFG, LINK, KEY, KEY, STA_CFG, SESSION];
/// Taking down contexts that reached the queues and the session.
const TEARDOWN_FROM_SESSION: [(u8, u8); 7] = [SESSION, FLUSH, QUEUE, QUEUE, STA_RM, LINK, PHY];

fn run_join(r: &Rig, policy: JoinPolicy, then: impl FnOnce(&mut crate::gen3::join::fw::Fw<'_, '_, crate::gen3_model::Model, crate::gen3_model::Mem, crate::gen3_model::Polls>, Result<Joined, JoinEnd>, Progress)) {
    let beacon = r.ap.borrow().beacon();
    let mut st = Station::new();
    with_fw(r, &mut st, |fw| {
        let mut p = Progress::default();
        let out = join(fw, &request(policy), &beacon, 6, 0x3, 0x3, &mut p);
        then(fw, out, p);
    });
}

fn joined(r: &Rig) -> (Akm, Progress) {
    let mut got = None;
    run_join(r, JoinPolicy::ANY, |_, out, p| {
        let j = out.unwrap_or_else(|e| panic!("joined, not {e:?}"));
        assert!(j.link.station.is_associated(), "the port is open");
        got = Some((j.akm, p));
    });
    got.unwrap()
}

// Every frame the station queued until the port opened was authentication or
// association on the management queue, or EAPOL in the clear on the data
// queue, all at the lowest basic rate with no firmware key.
fn only_handshake_frames_left(r: &Rig) {
    let sent = r.sent();
    assert!(!sent.is_empty());
    let (mgmt_q, data_q) = (sent[0].queue, sent.iter().find(|s| s.frame[0] == 0x08).map(|s| s.queue));
    for s in &sent {
        match s.frame[0] {
            0xB0 | 0x00 => assert_eq!(s.queue, mgmt_q, "management frames on the management queue"),
            0x08 => {
                assert!(is_clear_eapol(&s.frame), "a data frame before the port opened that is not EAPOL");
                assert_eq!(Some(s.queue), data_q);
            }
            other => panic!("frame {other:#x} left before the port opened"),
        }
        assert_eq!(s.flags, IWL_TX_FLAGS_CMD_RATE | IWL_TX_FLAGS_ENCRYPT_DIS);
        assert_eq!(s.rate, 0x4000, "1 Mb/s on antenna A, the lowest basic rate");
    }
    assert_ne!(Some(mgmt_q), data_q);
    assert_eq!(r.bc_mismatch(), 0, "every byte count entry matched its frame");
}

#[test]
fn a_wpa2_join_runs_linuxs_command_order_to_an_open_port() {
    let r = rig(Security::Wpa2, Switches::default());
    let (akm, p) = joined(&r);
    assert_eq!(akm, Akm::Psk);
    assert_eq!(r.commands(), JOIN_COMMANDS.to_vec());
    // Authentication, association, messages 2 and 4.
    let kinds: Vec<u8> = r.sent().iter().map(|s| s.frame[0]).collect();
    assert_eq!(kinds, vec![0xB0, 0x00, 0x08, 0x08]);
    only_handshake_frames_left(&r);
    assert_eq!((p.sent, p.eapol), (4, 2));

    // The keys in the firmware are the ones the access point derived.
    let ap = r.ap.borrow();
    assert!(ap.keyed, "the access point took message 4");
    let keys = r.command_payloads(0x5, 0x18);
    assert_eq!(&keys[0][16..32], &ap.tk(), "pairwise key, index 0");
    assert_eq!((keys[0][8], keys[0][12]), (0, 0x02), "no management protection on WPA2");
    assert_eq!(&keys[1][16..32], &GTK, "group key");
    assert_eq!((keys[1][8], keys[1][12]), (1, 0x22), "at the index the access point named");
    // Associated with AID 1, the station entry authorized without MFP.
    let macs = r.command_payloads(0x3, 0x08);
    assert_eq!((macs[0][36], macs[0][40]), (1, 1));
    let stas = r.command_payloads(0x3, 0x0A);
    assert_eq!((stas[0][36], stas[1][28], stas[1][36], stas[2][36]), (1, 1, 1, 0));
    // The link was activated on the PHY with the BSS's ACK rates, then took
    // the association's ERP preamble.
    let links = r.command_payloads(0x3, 0x09);
    assert_eq!((links[1][24], links[1][28], links[1][36], links[1][40]), (3, 1, 0x0F, 0x15));
    assert_eq!((links[2][24], links[2][44]), (2, 1));
    assert_eq!(&links[1][136..142], &[100, 0, 0, 0, 200, 0], "beacon interval and DTIM interval");
}

#[test]
fn a_wpa3_join_runs_sae_both_ways_to_an_open_port_with_protected_management() {
    for h2e in [true, false] {
        let r = rig(Security::Sae { h2e }, Switches::default());
        let (akm, _) = joined(&r);
        assert_eq!(akm, Akm::Sae, "h2e={h2e}");
        assert_eq!(r.commands(), JOIN_COMMANDS.to_vec());
        let kinds: Vec<u8> = r.sent().iter().map(|s| s.frame[0]).collect();
        assert_eq!(kinds, vec![0xB0, 0xB0, 0x00, 0x08, 0x08], "commit, confirm, association, 2, 4");
        only_handshake_frames_left(&r);
        let keys = r.command_payloads(0x5, 0x18);
        assert_eq!(&keys[0][16..32], &r.ap.borrow().tk());
        assert_eq!(keys[0][12], 0x42, "pairwise key with management protection");
        let stas = r.command_payloads(0x3, 0x0A);
        assert_eq!(stas[2][36], 1, "authorized with MFP");
    }
}

fn netif(op: u16, rid: u32, payload: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&wire::MAGIC_NNET.to_le_bytes());
    v.extend_from_slice(&wire::VERSION.to_le_bytes());
    v.extend_from_slice(&op.to_le_bytes());
    v.extend_from_slice(&[0; 4]);
    v.extend_from_slice(&rid.to_le_bytes());
    v.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    v.extend_from_slice(payload);
    v
}

fn status(out: &[u8]) -> i32 {
    i32::from_le_bytes(out[20..24].try_into().unwrap())
}

#[test]
fn data_crosses_both_ways_through_the_link_protocol() {
    for fw_decrypts in [false, true] {
        let sw = Switches { fw_decrypts, ..Switches::default() };
        let r = rig(Security::Wpa2, sw);
        run_join(&r, JoinPolicy::ANY, |fw, out, _| {
            let mut j = out.expect("joined");
            let tk = r.ap.borrow().tk();
            let mut port = Port { fw, link: &mut j.link };
            let mut out = vec![0u8; MAX_RESPONSE];
            let n = serve(&netif(wire::OP_LINK_STATUS, 1, &[]), &mut port, &mut out).unwrap();
            assert_eq!((status(&out), out[n - 1]), (0, 1), "link up");
            serve(&netif(wire::OP_MAC_ADDRESS, 2, &[]), &mut port, &mut out).unwrap();
            assert_eq!(&out[24..30], &STA);

            // Out: an IPv4 frame from the stack reaches the access point
            // protected under the pairwise key.
            let mut eth = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60];
            eth.extend_from_slice(&STA);
            eth.extend_from_slice(&[0x08, 0x00]);
            eth.extend_from_slice(b"dhcp discover");
            serve(&netif(wire::OP_TX_PACKET, 3, &eth), &mut port, &mut out).unwrap();
            assert_eq!(status(&out), 0);
            let last = r.sent().last().unwrap().clone();
            assert!(decrypts(&last.frame, &tk), "protected by the station");
            assert_eq!(last.flags, IWL_TX_FLAGS_CMD_RATE | IWL_TX_FLAGS_ENCRYPT_DIS, "no second encryption");
            assert_eq!(last.rate, 0x4003, "data at 11 Mb/s, the highest basic rate");
            let got = r.ap.borrow().got.last().unwrap().clone();
            assert_eq!(&got[6..], &[&[0x08, 0x00][..], b"dhcp discover"].concat()[..]);

            // In: the access point's protected reply comes up as Ethernet.
            let mut snap = vec![0xAA, 0xAA, 0x03, 0, 0, 0, 0x08, 0x00];
            snap.extend_from_slice(b"dhcp offer");
            let pkt = r.ap.borrow_mut().protected(&snap);
            r.model.queue(pkt);
            let n = serve(&netif(wire::OP_RX_PACKET, 4, &[]), &mut port, &mut out).unwrap();
            assert_eq!(status(&out), 0, "fw_decrypts={fw_decrypts}");
            let len = u32::from_le_bytes(out[24..28].try_into().unwrap()) as usize;
            let frame = &out[28..n];
            assert_eq!(frame.len(), len);
            assert_eq!(&frame[..6], &STA);
            assert_eq!(&frame[6..12], &AP);
            assert_eq!(&frame[12..], &[&[0x08, 0x00][..], b"dhcp offer"].concat()[..]);
            // The same frame again is a replay.
            r.model.queue(r.ap.borrow().deliver_replay());
            serve(&netif(wire::OP_RX_PACKET, 5, &[]), &mut port, &mut out).unwrap();
            assert_eq!(status(&out), wire::STATUS_AGAIN, "a replay is not delivered");
            assert_eq!(j.link.stats.rx_refused, 1);
        });
    }
}

#[test]
fn nothing_but_eapol_leaves_before_the_port_opens_whatever_arrives() {
    let r = rig(Security::Wpa2, Switches { silent_after_m1: true, ..Switches::default() });
    let beacon = r.ap.borrow().beacon();
    let mut st = Station::new();
    with_fw(&r, &mut st, |fw| {
        // Clear data that is not EAPOL, from the access point and from another
        // BSS, arrives during the handshake: none of it is answered.
        let mut snap = vec![0xAA, 0xAA, 0x03, 0, 0, 0, 0x08, 0x06];
        snap.extend_from_slice(&[0; 28]);
        let arp = from_ap(&snap, 9);
        let mut other = eapol_from_ap(&[0u8; 99], 10);
        other[10] ^= 1;
        r.model.queue(rx(&arp, 0));
        r.model.queue(rx(&other, 0));
        let mut p = Progress::default();
        let out = join(fw, &request(JoinPolicy::ANY), &beacon, 6, 3, 3, &mut p);
        assert_eq!(out.err(), Some(JoinEnd::TimedOut));
        assert!(p.data >= 2, "the clear data was received and dropped");
    });
    only_handshake_frames_left(&r);
}

#[test]
fn only_the_access_points_own_handshake_is_answered() {
    // Message 1 from another BSS arrives right after the access point's: one
    // message 2 answers the real one, and its resends are all that follow.
    let (r, e) = refused(Security::Wpa2, Switches { silent_after_m1: true, stray_m1: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::TimedOut);
    let m2s: Vec<_> = r.sent().into_iter().filter(|s| s.frame[0] == 0x08).collect();
    assert_eq!(m2s.len() as u32, 1 + IDLE_TRIES);
    assert!(m2s.iter().all(|s| s.frame == m2s[0].frame), "every one answers the access point's message 1");
}

#[test]
fn the_target_is_read_from_the_beacon_and_the_channel_it_was_heard_on() {
    let b = Ap::new(Security::Wpa2, Switches::default()).beacon();
    let t = target(&b, 11).unwrap();
    assert_eq!((t.bssid, t.channel, t.beacon_int, t.dtim_period), (AP, 6, 100, 2), "the DS element wins");
    assert_eq!((t.rates.cck_ack, t.rates.ofdm_ack, t.rates.mgmt, t.rates.data), (0x0F, 0x15, 0, 3));
    assert_eq!(t.capability, 0x0431);
    // No DS element (a 5 GHz beacon may carry none): the channel it was
    // heard on; heard on none, no target.
    let mut no_ds = b.clone();
    let at = no_ds.windows(3).position(|w| w == [3, 1, 6]).unwrap();
    no_ds.drain(at..at + 3);
    assert_eq!(target(&no_ds, 36).map(|t| t.channel), Some(36));
    assert_eq!(target(&no_ds, 0), None);
    assert_eq!(target(&b[..30], 6), None, "too short for its fixed fields");
}

#[test]
fn the_inbox_is_bounded_and_routes_transmit_responses_to_their_queue() {
    let mut q = Queues { mgmt: TxQueue::new(0), data: TxQueue::new(TXQ_STRIDE) };
    q.mgmt.start(4, 0);
    let mut inbox = Inbox::new();
    let f = from_ap(&[0xAA, 0xAA, 3, 0, 0, 0, 8, 0], 1);
    let pkt = rx(&f, 0);
    for _ in 0..INBOX_FRAMES + 1 {
        inbox.route(&Packet { cmd: pkt.cmd, group: 0, sequence: 0x8000, payload: &pkt.payload }, &mut q);
    }
    assert_eq!((inbox.frames.len(), inbox.dropped), (INBOX_FRAMES, 1));
    // A response for a queue the access point has not got is stray.
    let mut resp = vec![0u8; 48];
    resp[0] = 1;
    resp[36] = 9;
    resp[40] = 1;
    inbox.route(&Packet { cmd: 0x1C, group: 0, sequence: 0, payload: &resp }, &mut q);
    assert_eq!(inbox.stray, 1);
    resp[36] = 4;
    resp[44] = 5;
    inbox.route(&Packet { cmd: 0x1C, group: 0, sequence: 0, payload: &resp }, &mut q);
    assert_eq!(inbox.stray, 2, "naming a slot not in flight");
}

#[test]
fn networks_the_station_does_not_join_send_nothing_and_touch_no_context() {
    // An open network.
    let r = rig(Security::Wpa2, Switches::default());
    let mut st = Station::new();
    with_fw(&r, &mut st, |fw| {
        let open = beacon(&[]);
        let out = join(fw, &request(JoinPolicy::ANY), &open, 6, 3, 3, &mut Progress::default());
        assert_eq!(out.err(), Some(JoinEnd::Refused(MlmeFailure::OpenNetwork)));
        // Saved as WPA3, now offering WPA2 only.
        let wpa2 = beacon(&[&RSNE_PSK]);
        let out = join(fw, &request(JoinPolicy::WPA3_ONLY), &wpa2, 6, 3, 3, &mut Progress::default());
        assert_eq!(out.err(), Some(JoinEnd::Refused(MlmeFailure::Select(SelectError::Downgrade))));
        // Another network's beacon.
        let mut theirs = wpa2.clone();
        theirs[38] = b'H';
        let out = join(fw, &request(JoinPolicy::ANY), &theirs, 6, 3, 3, &mut Progress::default());
        assert_eq!(out.err(), Some(JoinEnd::NotTheNetwork));
        // The network's beacon, but with no channel in it and none it was
        // heard on.
        let mut nowhere = wpa2.clone();
        let at = nowhere.windows(3).position(|w| w == [3, 1, 6]).unwrap();
        nowhere.drain(at..at + 3);
        let out = join(fw, &request(JoinPolicy::ANY), &nowhere, 0, 3, 3, &mut Progress::default());
        assert_eq!(out.err(), Some(JoinEnd::NotTheNetwork));
    });
    assert!(r.commands().is_empty(), "no firmware context was put up");
    assert!(r.sent().is_empty(), "nothing was sent");
}

fn refused(sec: Security, sw: Switches) -> (Rig, JoinEnd) {
    let r = rig(sec, sw);
    let mut end = None;
    run_join(&r, JoinPolicy::ANY, |_, out, _| end = out.err());
    let e = end.expect("the join ended");
    (r, e)
}

// The commands after the join's setup, which must be the teardown.
fn after_setup(r: &Rig) -> Vec<(u8, u8)> {
    r.commands()[8..].to_vec()
}

#[test]
fn refusals_end_the_join_with_their_codes_and_take_the_contexts_down() {
    let (r, e) = refused(Security::Wpa2, Switches { refuse_auth: Some(17), ..Switches::default() });
    assert_eq!(e, JoinEnd::Refused(MlmeFailure::AuthRejected(17)));
    assert_eq!(after_setup(&r), TEARDOWN_FROM_SESSION.to_vec());

    let (r, e) = refused(Security::Wpa2, Switches { refuse_assoc: Some(18), ..Switches::default() });
    assert_eq!(e, JoinEnd::Refused(MlmeFailure::AssocRejected(18)));
    assert_eq!(after_setup(&r), TEARDOWN_FROM_SESSION.to_vec());

    let (r, e) = refused(Security::Wpa2, Switches { deauth_instead_of_m3: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::Refused(MlmeFailure::Left(15)));
    // Associated by then: the MAC context is marked unassociated again.
    let mut want = vec![MAC, STA_CFG, LINK, SESSION, MAC];
    want.extend_from_slice(&TEARDOWN_FROM_SESSION[1..]);
    assert_eq!(after_setup(&r), want);
    assert!(r.command_payloads(0x5, 0x18).is_empty(), "no key was installed");

    let (r, e) = refused(Security::Sae { h2e: true }, Switches { wrong_password: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::Refused(MlmeFailure::Sae(SaeFailure::BadConfirm)));
    assert_eq!(after_setup(&r), TEARDOWN_FROM_SESSION.to_vec());
}

#[test]
fn silences_time_out_with_the_unanswered_frame_resent() {
    let (r, e) = refused(Security::Wpa2, Switches { silent: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::TimedOut);
    let auths = r.sent().iter().filter(|s| s.frame[0] == 0xB0).count();
    assert_eq!(auths as u32, 1 + IDLE_TRIES, "sent, then resent at each silence");
    assert_eq!(after_setup(&r), TEARDOWN_FROM_SESSION.to_vec());

    let (r, e) = refused(Security::Wpa2, Switches { silent_after_auth: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::TimedOut);
    assert_eq!(r.sent().iter().filter(|s| s.frame[0] == 0x00).count() as u32, 1 + IDLE_TRIES);

    let (r, e) = refused(Security::Wpa2, Switches { silent_after_m1: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::TimedOut);
    assert_eq!(r.sent().iter().filter(|s| s.frame[0] == 0x08).count() as u32, 1 + IDLE_TRIES, "message 2");

    // A wrong WPA2 passphrase: message 3 fails its MIC at the station and is
    // dropped; the join times out with no key installed.
    let (r, e) = refused(Security::Wpa2, Switches { wrong_pmk: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::TimedOut);
    assert!(r.command_payloads(0x5, 0x18).is_empty());
    assert!(!r.ap.borrow().keyed);
}

#[test]
fn a_firmware_that_will_not_put_the_radio_on_channel_ends_the_join_before_any_frame() {
    for (reply, want) in [
        (SessionReply::Refuse, JoinEnd::Setup(SetupError::NoSession(Some(Session::Refused)))),
        (SessionReply::Never, JoinEnd::Setup(SetupError::NoSession(None))),
    ] {
        let r = rig_with(Security::Wpa2, Switches::default(), reply);
        let mut end = None;
        run_join(&r, JoinPolicy::ANY, |_, out, _| end = out.err());
        assert_eq!(end, Some(want));
        assert!(r.sent().is_empty(), "nothing was sent");
        assert_eq!(after_setup(&r), TEARDOWN_FROM_SESSION[1..].to_vec(), "no session to cancel");
    }
}

#[test]
fn a_bad_queue_reply_or_a_firmware_error_takes_down_what_was_up() {
    let r = rig(Security::Wpa2, Switches::default());
    r.model.s.borrow_mut().txq.as_mut().unwrap().queue_reply_len = 12;
    let mut end = None;
    run_join(&r, JoinPolicy::ANY, |_, out, _| end = out.err());
    assert_eq!(end, Some(JoinEnd::Setup(SetupError::BadQueueReply)));
    assert_eq!(r.commands(), vec![PHY, RLC, LINK, LINK, STA_CFG, QUEUE, STA_RM, LINK, PHY]);

    let r = rig(Security::Wpa2, Switches::default());
    // The firmware asserts on the join's first command.
    r.model.s.borrow_mut().fw_error_after = Some(0);
    let mut end = None;
    run_join(&r, JoinPolicy::ANY, |_, out, _| end = out.err());
    match end {
        Some(JoinEnd::Setup(SetupError::Command(c))) => assert_eq!((c.group, c.cmd), (0x1, 0x08)),
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_key_the_firmware_does_not_take_ends_the_join_and_removes_nothing_it_did_not_install() {
    let r = rig(Security::Wpa2, Switches::default());
    r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd, c.payload.get(4)) {
        (0x5, 0x18, _) => vec![],
        (0x3, 0x05, Some(1)) => vec![reply(c, vec![]), session_notif(1, 1)],
        _ => vec![reply(c, vec![])],
    }));
    let mut end = None;
    run_join(&r, JoinPolicy::ANY, |_, out, _| end = out.err());
    assert!(matches!(end, Some(JoinEnd::Keys(KeyError::Pairwise(_)))), "{end:?}");
    let cmds = r.commands();
    assert_eq!(cmds.iter().filter(|c| **c == KEY).count(), 1, "only the add that went unanswered");
    let tail = vec![KEY, SESSION, MAC, FLUSH, QUEUE, QUEUE, STA_RM, LINK, PHY];
    assert_eq!(cmds[cmds.len() - tail.len()..].to_vec(), tail, "no key removal, then the teardown");
}

#[test]
fn a_transmit_response_is_never_taken_for_a_commands_reply() {
    let r = rig(Security::Wpa2, Switches::default());
    // The firmware answers with a TX response carrying the command's own
    // sequence first, then the real reply.
    r.model.s.borrow_mut().responder = Some(Box::new(|c: &Seen| {
        let mut resp = vec![0u8; 48];
        resp[0] = 1;
        vec![Out { cmd: 0x1C, group: 0, seq: c.seq, payload: resp }, reply(c, vec![7, 7])]
    }));
    let mut st = Station::new();
    with_fw(&r, &mut st, |fw| {
        assert_eq!(fw.command(0x3, 0x0C, &[0; 4]), Ok(vec![7, 7]));
    });
}

#[test]
fn leaving_tells_the_access_point_removes_the_keys_and_takes_everything_down() {
    for (sec, protected) in [(Security::Wpa2, false), (Security::Sae { h2e: true }, true)] {
        let r = rig(sec, Switches::default());
        run_join(&r, JoinPolicy::ANY, |fw, out, _| {
            let mut j = out.expect("joined");
            leave(fw, &mut j);
            assert!(!j.link.station.is_associated());
            assert_eq!(j.bss.stage, Stage::Down);
        });
        let ap = r.ap.borrow();
        assert_eq!(ap.heard.iter().filter(|f| f[0] == 0xC0).count(), 1);
        let deauth = ap.heard.iter().find(|f| f[0] == 0xC0).unwrap();
        assert_eq!(deauth[1] & 0x40 != 0, protected, "protected under the pairwise key with MFP");
        let tail = r.commands()[JOIN_COMMANDS.len()..].to_vec();
        assert_eq!(tail, vec![KEY, KEY, MAC, FLUSH, QUEUE, QUEUE, STA_RM, LINK, PHY]);
        let keys = r.command_payloads(0x5, 0x18);
        assert_eq!((keys[2][0], keys[2][8]), (3, 1), "the group key removed");
        assert_eq!((keys[3][0], keys[3][8]), (3, 0), "then the pairwise key");
    }
}

#[test]
fn the_access_points_deauthentication_takes_the_link_down_unless_it_could_be_forged() {
    // Without management frame protection an unprotected one counts.
    let r = rig(Security::Wpa2, Switches::default());
    run_join(&r, JoinPolicy::ANY, |fw, out, _| {
        let mut j = out.expect("joined");
        let mut port = Port { fw, link: &mut j.link };
        r.model.queue(rx(&leave_frame(), 0));
        assert!(port.poll_rx(&mut [0u8; 2048]).is_none());
        assert!(!port.link_up());
        assert_eq!(j.link.left, Some(15));
    });
    // With it, an unprotected one is ignored; one the firmware decrypted
    // (its MIC held) counts.
    let r = rig(Security::Sae { h2e: true }, Switches { fw_decrypts: true, ..Switches::default() });
    run_join(&r, JoinPolicy::ANY, |fw, out, _| {
        let mut j = out.expect("joined");
        let mut port = Port { fw, link: &mut j.link };
        r.model.queue(rx(&leave_frame(), 0));
        let _ = port.poll_rx(&mut [0u8; 2048]);
        assert!(port.link_up(), "a forged deauthentication is ignored");
        let pkt = r.ap.borrow_mut().protected_mgmt(&leave_frame());
        r.model.queue(pkt);
        let _ = port.poll_rx(&mut [0u8; 2048]);
        assert!(!port.link_up());
        assert_eq!(j.link.left, Some(15));
    });
}

fn leave_frame() -> Vec<u8> {
    let mut f = vec![0xC0, 0, 0, 0];
    f.extend_from_slice(&STA);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&[0, 0, 15, 0]);
    f
}

#[test]
fn a_group_rekey_is_answered_protected_and_moves_the_key_in_the_firmware() {
    let r = rig(Security::Wpa2, Switches::default());
    run_join(&r, JoinPolicy::ANY, |fw, out, _| {
        let mut j = out.expect("joined");
        let new_gtk = [0x77u8; 16];
        let ptk = r.ap.borrow().ptk;
        let msg = group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &new_gtk));
        let mut snap = vec![0xAA, 0xAA, 0x03, 0, 0, 0, 0x88, 0x8E];
        snap.extend_from_slice(&msg);
        let pkt = r.ap.borrow_mut().protected(&snap);
        r.model.queue(pkt);
        {
            let mut port = Port { fw, link: &mut j.link };
            port.service();
        }
        assert_eq!(j.link.new_group_key, Some((2, new_gtk)));
        let replies = r.ap.borrow().eapol_replies.len();
        assert_eq!(replies, 3, "messages 2 and 4, then group message 2");
        let last = r.sent().last().unwrap().clone();
        assert!(decrypts(&last.frame, &r.ap.borrow().tk()), "answered protected");
        let (id, key) = j.link.new_group_key.take().unwrap();
        j.keys.rekey(fw, &key, id).expect("installed");
        assert_eq!(j.keys.group, Some(2));
    });
    let keys = r.command_payloads(0x5, 0x18);
    assert_eq!((keys[2][0], keys[2][8], &keys[2][16..32]), (1, 2, &[0x77u8; 16][..]), "the new key at index 2");
    assert_eq!((keys[3][0], keys[3][8]), (3, 1), "then the old one at index 1 removed");
}

#[test]
fn a_session_that_ends_before_the_port_opens_is_asked_for_again() {
    let r = rig(Security::Wpa2, Switches::default());
    let beacon = r.ap.borrow().beacon();
    let mut st = Station::new();
    with_fw(&r, &mut st, |fw| {
        // The session ends as soon as it started.
        let mut p = Progress::default();
        let first = std::cell::Cell::new(true);
        r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd, c.payload.get(4)) {
            (0x3, 0x05, Some(1)) if first.replace(false) => {
                vec![reply(c, vec![]), session_notif(1, 1), session_notif(1, 0)]
            }
            (0x3, 0x05, Some(1)) => vec![reply(c, vec![]), session_notif(1, 1)],
            _ => vec![reply(c, vec![])],
        }));
        let j = join(fw, &request(JoinPolicy::ANY), &beacon, 6, 3, 3, &mut p).expect("joined");
        assert!(j.link.station.is_associated());
    });
    let sessions = r.command_payloads(0x3, 0x05);
    assert_eq!(sessions.iter().filter(|s| s[4] == 1).count(), 2, "asked for again");
    assert_eq!(sessions.last().unwrap()[4], 3, "cancelled once the port opened");
}
