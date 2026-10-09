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

//! The join's firmware commands, byte for byte.
//!
//! Each image below is the structure its builder names (fw/api/phy-ctxt.h,
//! mac-cfg.h, time-event.h, datapath.h, scan.h of Linux v6.12), with every
//! field Linux fills for a legacy station join written out at its offset.
//! The versions are held to what the bundled firmware files report in their
//! command version table: a firmware listing another version of any of
//! these commands is refused before anything is sent.

use crate::gen3::cmds::{link_config_add, mac_config_add};
use crate::gen3::station::abort::scan_abort;
use crate::gen3::station::ids::{check, APIS, PHY_CONTEXT_CMD, SEC_KEY_CMD};
use crate::gen3::station::key::{add_key, remove_key, KeyKind};
use crate::gen3::station::link::{link_config, LinkParams, MODIFY_ACTIVE, MODIFY_RATES_INFO};
use crate::gen3::station::mac::mac_config;
use crate::gen3::station::phy::{phy_context, rlc_config, ACTION_ADD, ACTION_MODIFY, ACTION_REMOVE};
use crate::gen3::station::queue::{add_queue, cb_size, flush_sta, parse_queue_reply, remove_queue, QueueGiven};
use crate::gen3::station::rates::{bss_rates, rate_n_flags, BssRates};
use crate::gen3::station::session::{parse_notif, session_protection, Session};
use crate::gen3::station::sta::{sta_config, sta_remove};
use crate::gen3::ucode::Ucode;

static SO_GF: &[u8] = include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");
static SO_HR: &[u8] = include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-hr-b0-84.ucode");

const ADDR: [u8; 6] = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
const BSSID: [u8; 6] = [0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];

// `n` zero bytes after `head`.
fn image(head: &[u8], n: usize) -> Vec<u8> {
    let mut v = head.to_vec();
    v.resize(n, 0);
    v
}

fn le32(v: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(v[at..at + 4].try_into().unwrap())
}

// The firmware file with the command version table entry for `(group, cmd)`
// rewritten to `version`.
fn with_version(blob: &[u8], group: u8, cmd: u8, version: u8) -> Vec<u8> {
    let mut b = blob.to_vec();
    let mut off = 88;
    while off + 8 <= b.len() {
        let ty = le32(&b, off);
        let len = le32(&b, off + 4) as usize;
        if ty == 48 {
            for e in (off + 8..off + 8 + len - len % 4).step_by(4) {
                if b[e] == cmd && b[e + 1] == group {
                    b[e + 2] = version;
                    return b;
                }
            }
        }
        off += 8 + ((len + 3) & !3);
    }
    panic!("no entry for {group:#x}/{cmd:#x}");
}

#[test]
fn both_bundled_firmwares_speak_every_layout_the_join_encodes() {
    for blob in [SO_GF, SO_HR] {
        let u = Ucode::parse(blob).unwrap();
        assert_eq!(check(&u), Ok(()));
        for a in &APIS {
            match u.cmd_version(a.group, a.cmd) {
                Some(v) => assert!(a.listed.contains(&v), "{:#x}/{:#x} at {v}", a.group, a.cmd),
                None => assert!(a.listed.is_empty(), "{:#x}/{:#x} unlisted", a.group, a.cmd),
            }
        }
    }
    // The so-a0-gf image runs session protection version 2, the hr image 1:
    // the bytes are the same for MAC 0 and link 0.
    let gf = Ucode::parse(SO_GF).unwrap();
    let hr = Ucode::parse(SO_HR).unwrap();
    assert_eq!((gf.cmd_version(3, 5), hr.cmd_version(3, 5)), (Some(2), Some(1)));
}

#[test]
fn a_firmware_with_another_layout_is_refused() {
    let moved = with_version(SO_GF, 1, PHY_CONTEXT_CMD, 5);
    assert_eq!(check(&Ucode::parse(&moved).unwrap()), Err((1, PHY_CONTEXT_CMD)));
    let moved = with_version(SO_GF, 5, SEC_KEY_CMD, 2);
    assert_eq!(check(&Ucode::parse(&moved).unwrap()), Err((5, SEC_KEY_CMD)));
}

#[test]
fn the_phy_context_names_the_channel_band_and_width() {
    let add = phy_context(ACTION_ADD, 6);
    assert_eq!(add.to_vec(), image(&[0, 0, 0, 0, 1, 0, 0, 0, 6, 0, 0, 0, 1, 0, 0, 0], 32));
    let five = phy_context(ACTION_ADD, 149);
    assert_eq!(five.to_vec(), image(&[0, 0, 0, 0, 1, 0, 0, 0, 149, 0, 0, 0, 0, 0, 0, 0], 32));
    let remove = phy_context(ACTION_REMOVE, 6);
    assert_eq!(remove.to_vec(), image(&[0, 0, 0, 0, 3, 0, 0, 0, 6, 0, 0, 0, 1, 0, 0, 0], 32));
}

#[test]
fn the_rlc_config_carries_the_receive_chains() {
    // Two antennas: valid 0b11 at bit 1, two idle and two active chains.
    assert_eq!(rlc_config(3).to_vec(), image(&[0, 0, 0, 0, 0x06, 0x28, 0, 0], 32));
    // One antenna: one chain of each.
    assert_eq!(rlc_config(1).to_vec(), image(&[0, 0, 0, 0, 0x02, 0x14, 0, 0], 32));
}

#[test]
fn the_mac_context_without_association_is_the_bring_ups_add() {
    assert_eq!(mac_config(ACTION_ADD, ADDR, None), mac_config_add(ADDR));
    let mut want = vec![0u8; 52];
    want[4] = 2;
    want[8] = 5;
    want[12..18].copy_from_slice(&ADDR);
    want[20] = 0x04; // group frames, no longer beacons
    want[32] = 1;
    want[36] = 1; // is_assoc
    want[40..42].copy_from_slice(&0x0123u16.to_le_bytes());
    assert_eq!(mac_config(ACTION_MODIFY, ADDR, Some(0x0123)).to_vec(), want);
    let off = mac_config(ACTION_MODIFY, ADDR, None);
    assert_eq!((off[20], off[36], off[40]), (0x0C, 0, 0), "left: beacons accepted again");
}

#[test]
fn the_link_context_add_matches_the_bring_up_and_a_change_says_what_changed() {
    let add = link_config(ACTION_ADD, &LinkParams { addr: ADDR, ..LinkParams::default() });
    assert_eq!(add, link_config_add(ADDR));
    let p = LinkParams {
        addr: ADDR,
        phy: Some(0),
        active: true,
        modify_mask: MODIFY_ACTIVE | MODIFY_RATES_INFO,
        cck_ack: 0x0F,
        ofdm_ack: 0x15,
        short_preamble: true,
        short_slot: true,
        beacon_int: 100,
        dtim_period: 3,
    };
    let mut want = vec![0u8; 208];
    want[0] = 2;
    want[16..22].copy_from_slice(&ADDR);
    want[24] = 3;
    want[28] = 1;
    want[36] = 0x0F;
    want[40] = 0x15;
    want[44] = 1;
    want[48] = 1;
    want[136] = 100;
    want[140..142].copy_from_slice(&300u16.to_le_bytes());
    assert_eq!(link_config(ACTION_MODIFY, &p), want);
    let off = link_config(ACTION_MODIFY, &LinkParams { active: false, modify_mask: MODIFY_ACTIVE, ..p });
    assert_eq!((le32(&off, 12), le32(&off, 24), le32(&off, 28)), (0, 1, 0), "deactivated, PHY kept");
}

#[test]
fn ack_rates_add_the_mandatory_rates_below_the_lowest_basic_one() {
    // An 802.11b/g AP: 1, 2, 5.5 and 11 basic, the OFDM rates not.
    let bg = [0x82, 0x84, 0x8B, 0x96, 0x0C, 0x12, 0x18, 0x24, 0x30, 0x48, 0x60, 0x6C];
    assert_eq!(bss_rates(&bg, 6), BssRates { cck_ack: 0x0F, ofdm_ack: 0x15, mgmt: 0, data: 3 });
    // A g-only AP: 6, 12 and 24 basic; CCK all added as mandatory.
    let g = [0x8C, 0x12, 0x98, 0x24, 0xB0, 0x48, 0x60, 0x6C];
    assert_eq!(bss_rates(&g, 1), BssRates { cck_ack: 0x0F, ofdm_ack: 0x15, mgmt: 4, data: 8 });
    // 11 basic only: 1, 2 and 5.5 below it are added.
    assert_eq!(bss_rates(&[0x96, 0x0C], 11).cck_ack, 0x0F);
    // 5 GHz with 12 and 24 basic: 6 is mandatory below; a CCK octet is not a
    // rate there.
    let a = [0x82, 0x98, 0xB0, 0x48];
    assert_eq!(bss_rates(&a, 36), BssRates { cck_ack: 0x0F, ofdm_ack: 0x15, mgmt: 6, data: 8 });
    // 54 basic: 6, 12 and 24 below it.
    assert_eq!(bss_rates(&[0xEC], 36).ofdm_ack, 0x80 | 0x15);
    // No basic rate at all: the lowest of the band.
    assert_eq!((bss_rates(&[0x0C], 1).mgmt, bss_rates(&[0x0C], 40).mgmt), (0, 4));
}

#[test]
fn the_rate_word_is_the_version_2_legacy_format() {
    assert_eq!(rate_n_flags(0, 0x3), 0x0000_4000, "1 Mb/s CCK on antenna A");
    assert_eq!(rate_n_flags(3, 0x3), 0x0000_4003, "11 Mb/s CCK");
    assert_eq!(rate_n_flags(4, 0x3), 0x0000_4100, "6 Mb/s OFDM");
    assert_eq!(rate_n_flags(8, 0x2), 0x0000_8104, "24 Mb/s OFDM on antenna B");
    assert_eq!(rate_n_flags(11, 0x1), 0x0000_4107, "54 Mb/s OFDM");
    assert_eq!(rate_n_flags(99, 0x0), 0x0000_4107, "an index past 54 Mb/s is 54, no antenna is A");
}

#[test]
fn session_protection_asks_for_900_ms_and_cancels_with_the_same_session() {
    let add = session_protection(ACTION_ADD, 900);
    assert_eq!(add.to_vec(), image(&[0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0x6E, 0x03, 0, 0], 24));
    let remove = session_protection(ACTION_REMOVE, 0);
    assert_eq!(remove.to_vec(), image(&[0, 0, 0, 0, 3, 0, 0, 0], 24));
    let notif = |link: u32, status: u32, start: u32, conf: u32| {
        [link, status, start, conf].iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<u8>>()
    };
    assert_eq!(parse_notif(&notif(0, 1, 1, 0)), Some(Session::Started));
    assert_eq!(parse_notif(&notif(0, 1, 0, 0)), Some(Session::Ended));
    assert_eq!(parse_notif(&notif(0, 0, 0, 0)), Some(Session::Refused));
    assert_eq!(parse_notif(&notif(1, 1, 1, 0)), None, "another link");
    assert_eq!(parse_notif(&notif(0, 1, 1, 4)), None, "another session");
    assert_eq!(parse_notif(&notif(0, 1, 1, 0)[..15]), None, "short");
}

#[test]
fn the_access_point_station_entry() {
    let mut want = vec![0u8; 96];
    want[8..14].copy_from_slice(&BSSID);
    want[16..22].copy_from_slice(&BSSID);
    want[36] = 1;
    assert_eq!(sta_config(0, BSSID, 0, true).to_vec(), want, "added, before authorization");
    want[28..30].copy_from_slice(&7u16.to_le_bytes());
    want[36] = 0;
    assert_eq!(sta_config(0, BSSID, 7, false).to_vec(), want, "authorized without protection");
    assert_eq!(sta_remove(), [0, 0, 0, 0]);
}

#[test]
fn keys_carry_the_cipher_the_group_bit_and_management_protection() {
    let tk = [0x5Au8; 16];
    let mut want = vec![0u8; 80];
    want[0] = 1;
    want[4] = 1;
    want[12] = 0x42; // CCMP, MFP
    want[16..32].copy_from_slice(&tk);
    assert_eq!(add_key(1, 0, KeyKind::Pairwise { mfp: true }, &tk).to_vec(), want);
    want[12] = 0x02;
    assert_eq!(add_key(1, 0, KeyKind::Pairwise { mfp: false }, &tk).to_vec(), want);
    let gtk = [0xA5u8; 16];
    want[8] = 2;
    want[12] = 0x22; // CCMP, multicast
    want[16..32].copy_from_slice(&gtk);
    assert_eq!(add_key(1, 2, KeyKind::Group, &gtk).to_vec(), want);
    let remove = remove_key(1, 2, KeyKind::Group);
    assert_eq!(remove.to_vec(), image(&[3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0x22, 0, 0, 0], 80));
}

#[test]
fn queues_are_asked_for_by_tid_and_their_reply_is_exactly_eight_bytes() {
    let add = add_queue(1, 15, 128, 0x1122_3344_5566_7788, 0x0102_0304_0506_0708);
    let mut want = vec![0u8; 36];
    want[4] = 1;
    want[8] = 15;
    want[16] = 4;
    want[20..28].copy_from_slice(&0x1122_3344_5566_7788u64.to_le_bytes());
    want[28..36].copy_from_slice(&0x0102_0304_0506_0708u64.to_le_bytes());
    assert_eq!(add.to_vec(), want);
    assert_eq!(remove_queue(1, 0).to_vec(), image(&[1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0], 36));
    assert_eq!((cb_size(128), cb_size(256), cb_size(16)), (4, 5, 1));
    let reply = [9, 0, 0, 0, 0x34, 0x12, 0, 0];
    assert_eq!(parse_queue_reply(&reply), Some(QueueGiven { queue: 9, write_ptr: 0x1234 }));
    assert_eq!(parse_queue_reply(&reply[..7]), None);
    assert_eq!(parse_queue_reply(&[0u8; 12]), None);
    assert_eq!(flush_sta(), [0, 0, 0, 0, 0xFF, 0xFF, 0, 0]);
}

#[test]
fn a_scan_is_aborted_by_its_uid() {
    assert_eq!(scan_abort(), [0u8; 8]);
}
