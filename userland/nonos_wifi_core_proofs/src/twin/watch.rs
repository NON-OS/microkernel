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

//! What the twin does to the air and what it reads off it: a frame put
//! there, the net stack's own send, what the station delivered and sent.

use crate::soft::join::{sta_of, ticks};
use crate::soft::rig::Station;

/// Put the frame `make` builds on the air and let the station take it.
pub fn put(st: &mut Station, make: impl FnOnce(&Station) -> Vec<u8>) {
    let f = make(st);
    st.radio_mut().unwrap().ap.chatter = false;
    st.radio_mut().unwrap().inbox.push_back(f);
    ticks(st, 3);
}

/// The net stack sends one IPv4 packet to the AP's network.
pub fn send(st: &mut Station) {
    let eth = [&[2, 0, 0, 0, 0, 9][..], &sta_of(st), &[8, 0], b"to the network"].concat();
    crate::soft::wire::netif(st, nonos_wifi_core::netif::wire::OP_TX_PACKET, &eth);
}

/// Ethernet frames the station has handed the stack.
pub fn delivered(st: &Station) -> u32 {
    st.stats().rx_eth
}

/// The packet numbers of the station's protected data frames, in order.
pub fn sent_pns(st: &Station) -> Vec<u64> {
    st.radio()
        .unwrap()
        .sent
        .iter()
        .filter(|f| (f[0] >> 2) & 3 == 2 && f[1] & 0x40 != 0)
        .map(|f| {
            let h = if f[0] & 0x80 != 0 { 26 } else { 24 };
            let p = &f[h..h + 8];
            u64::from_le_bytes([p[0], p[1], p[4], p[5], p[6], p[7], 0, 0])
        })
        .collect()
}
