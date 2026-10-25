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

//! What anyone on the air can send. Seeded random management frames, and
//! every cut of every protected frame the access point sends, never panic and
//! never end the link, and another network's beacons
//! keep nothing alive.

use super::frames::{bip_signed, mgmt, mgmt_from, protected, ACTION, BEACON, DEAUTH};
use super::sim::{joined, IGTK, STA};
use nonos_wifi_core::dot11::header::BROADCAST;
use nonos_wifi_core::station::{Rx, Tick, QUIET_MS};

#[test]
fn random_and_cut_management_frames_never_panic_or_end_the_link() {
    let mut s = joined(true);
    let mut x = 0x9E37_79B9_7F4A_7C15u64;
    for n in 0..20_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let mut f = mgmt(DEAUTH, STA, &x.to_le_bytes());
        f[0] = (x as u8) & 0xFC; // any management subtype
        f[1] = (x >> 8) as u8 & !0x03;
        f.truncate((x >> 16) as usize % (f.len() + 1));
        // Never chip-decrypted: a chip vouches only for a MIC it checked.
        let r = s.receive(&f, false);
        assert!(!matches!(r, Rx::Lost(_)), "frame {n}: {f:02x?}");
    }
    assert!(s.is_associated());
    let real = [
        protected(&mgmt(DEAUTH, STA, &[3, 0]), 1),
        bip_signed(&mgmt(DEAUTH, BROADCAST, &[3, 0]), 4, 9, &IGTK),
        protected(&mgmt(ACTION, STA, &[8, 0, 1, 0]), 2),
    ];
    for f in &real {
        for cut in 0..f.len() {
            let mut s = joined(true);
            assert!(!matches!(s.receive(&f[..cut], false), Rx::Lost(_)), "cut to {cut}");
        }
    }
}

#[test]
fn another_network_s_beacons_keep_nothing_alive() {
    let mut s = joined(false);
    s.tick(0);
    for t in (100..QUIET_MS).step_by(100) {
        s.receive(&mgmt_from([0x02, 0, 0, 0, 0, 9], BEACON, BROADCAST, &[0; 12]), false);
        assert_eq!(s.tick(t), Tick::Quiet);
    }
    assert!(matches!(s.tick(QUIET_MS), Tick::Send(_)), "asked after the quiet");
}
