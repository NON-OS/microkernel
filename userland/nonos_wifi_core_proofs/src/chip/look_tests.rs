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
//! A look, away from a weak AP to listen for a stronger one of the network,
//! through a chip that lies between frames: the station goes, comes back
//! or moves, and holds a session either way.

use super::rig::{connect, station};
use crate::roam::two_aps::BB;
use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{PASS, SSID};

#[test]
fn a_look_through_a_lying_chip_keeps_a_session() {
    for seed in [3u64, 0x5eed, 0xabcd_ef01, 0x1234_5678_9abc] {
        let mut st = station(seed, false);
        assert_eq!(connect(&mut st), 0);
        let mut b = SimAp::new(SSID, PASS, 1, &RSNE_PSK);
        (b.bssid, b.chatter) = (BB, true);
        let air = &mut st.radio_mut().unwrap().air;
        (air.ap2, air.rssi) = (Some(b), [-85, -50]);
        let looks = |st: &super::rig::Lied| {
            st.radio().unwrap().air.sent.iter().filter(|f| f[0] == 0x48).count()
        };
        (0..3000).for_each(|_| st.tick());
        assert!(looks(&st) > 0, "it looked");
        assert!(st.session().is_some(), "and holds a session");
    }
}
