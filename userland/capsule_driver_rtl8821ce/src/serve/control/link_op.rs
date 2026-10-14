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

//! The link op: whether the radio is associated and with which network, as
//! `[associated u8][bssid 6][ssid_len u8][ssid][akm u8]` after the header,
//! the AKM 0 when not associated, then the link's history (`LinkHistory`). The
//! panels read it to show the connection whoever made it (a panel, or net_core
//! joining a saved network at boot), and the client reads the AKM (8 for
//! WPA3-SAE) to remember a network as WPA3 so it is never joined with WPA2.

use nonos_wifi_core::link_history::LinkHistory;
use nonos_wifi_core::netif::LinkPort;

use super::super::connect::Session;
use super::super::radio::Radio;
use super::WIFI_HDR;

pub(super) fn link_reply(
    radio: &Radio,
    session: Option<&Session>,
    out: &mut [u8],
) -> Option<usize> {
    let up = matches!(radio, Radio::Up(up) if up.link.link_up());
    let history = match radio {
        Radio::Up(up) => up.link.history(),
        Radio::Down => LinkHistory::NEW,
    };
    let fixed = WIFI_HDR + 8;
    out.get_mut(WIFI_HDR..=fixed)?.fill(0);
    let mut end = fixed + 1;
    if let Some(s) = session.filter(|_| up) {
        let len = (s.ssid_len as usize).min(s.ssid.len());
        out[WIFI_HDR] = 1;
        out[WIFI_HDR + 1..WIFI_HDR + 7].copy_from_slice(&s.bssid);
        out[fixed - 1] = len as u8;
        out.get_mut(fixed..fixed + len)?.copy_from_slice(&s.ssid[..len]);
        *out.get_mut(fixed + len)? = s.akm;
        end = fixed + len + 1;
    }
    Some(end + history.encode(out.get_mut(end..)?)?)
}
