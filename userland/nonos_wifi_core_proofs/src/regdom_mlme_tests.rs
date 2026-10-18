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

//! A join under a regulatory domain: an access point heard on a channel the
//! domain closes is treated as not heard, one on a passive channel is joined
//! (its beacon is the enabling signal), and a narrower domain is honoured.

use nonos_wifi_core::dot11::header::{frame_control, write_header, MAC_HEADER_LEN, TYPE_MGMT};
use nonos_wifi_core::mlme::{Mlme, MlmeState};
use nonos_wifi_core::regdom::{parse_country, WORLD};

const STA: [u8; 6] = [0x02, 0, 0, 0, 0, 0x02];
const AP: [u8; 6] = [0x02, 0, 0, 0, 0, 0x01];
const SSID: &[u8] = b"home";
// RSN: CCMP group and pairwise, PSK.
const RSNE: [u8; 22] =
    [48, 20, 1, 0, 0x00, 0x0f, 0xac, 4, 1, 0, 0x00, 0x0f, 0xac, 4, 1, 0, 0x00, 0x0f, 0xac, 2, 0, 0];

fn beacon_on(channel: u8) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, 8), [0xFF; 6], AP, AP, 0).unwrap();
    f.extend_from_slice(&[0u8; 8]);
    f.extend_from_slice(&[0x64, 0, 0x11, 0x04]);
    f.extend_from_slice(&[0, SSID.len() as u8]);
    f.extend_from_slice(SSID);
    f.extend_from_slice(&[1, 4, 0x82, 0x84, 0x8b, 0x96]);
    f.extend_from_slice(&[3, 1, channel]);
    f.extend_from_slice(&RSNE);
    f
}

fn join() -> Mlme {
    Mlme::new(STA, SSID, b"correct horse", [0x52; 32])
}

#[test]
fn an_ap_on_a_closed_channel_is_not_joined() {
    for ch in [14, 169] {
        let mut m = join();
        assert!(m.on_mgmt(&beacon_on(ch)).tx.is_none(), "channel {ch}");
        assert_eq!(m.state(), MlmeState::Scanning);
    }
}

#[test]
fn an_ap_on_a_passive_channel_is_joined_after_its_beacon() {
    for ch in [13, 36, 100] {
        let mut m = join();
        assert!(m.on_mgmt(&beacon_on(ch)).tx.is_some(), "channel {ch}");
        assert_eq!((m.state(), m.channel()), (MlmeState::Authenticating, ch));
    }
}

#[test]
fn a_narrower_domain_closes_what_it_does_not_list() {
    let mut m = join();
    assert_eq!(m.domain(), WORLD);
    m.set_domain(WORLD.restrict(&parse_country(&[b'D', b'E', b' ', 1, 5, 20]).unwrap()));
    assert!(m.on_mgmt(&beacon_on(6)).tx.is_none());
    assert!(m.on_mgmt(&beacon_on(5)).tx.is_some());
}
