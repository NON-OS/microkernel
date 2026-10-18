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

//! Deauthentication on a link with management frame protection: a forged
//! one in the clear is dropped (or, claiming the AP lost the association,
//! starts an SA Query), and only one under the pairwise key or a valid BIP
//! MIC above the IPN floor ends the link.

use crate::ap_sim::SPA;
use crate::pmf_frames::{mgmt, sealed};
use crate::pmf_sim::{joined_pmf, IGTK};
use nonos_wifi_core::pmf::bip_protect;
use nonos_wifi_core::station::{MgmtDrop, MgmtRx};

const DEAUTH: u8 = 12;
const DISASSOC: u8 = 10;
const BCAST: [u8; 6] = [0xFF; 6];

#[test]
fn a_deauthentication_in_the_clear_does_not_end_a_protected_link() {
    let (mut s, _) = joined_pmf();
    let forged = mgmt(DEAUTH, SPA, &3u16.to_le_bytes());
    assert_eq!(s.receive_mgmt(&forged, false), MgmtRx::Dropped(MgmtDrop::Unprotected));
    let lost = mgmt(DISASSOC, SPA, &7u16.to_le_bytes());
    assert_eq!(s.receive_mgmt(&lost, false), MgmtRx::SaQueryNeeded);
    assert_eq!(
        s.receive_mgmt(&mgmt(DEAUTH, BCAST, &[1, 0]), false),
        MgmtRx::Dropped(MgmtDrop::Undecryptable)
    );
    assert!(s.is_associated());
}

#[test]
fn a_protected_deauthentication_ends_the_link_once() {
    for hw in [false, true] {
        let (mut s, ptk) = joined_pmf();
        let f = sealed(&mgmt(DEAUTH, SPA, &[1, 0]), 1, &ptk, hw);
        assert_eq!(s.receive_mgmt(&f, hw), MgmtRx::Left(1), "hw {hw}");
        assert!(!s.is_associated());
        assert_eq!(s.receive_mgmt(&f, hw), MgmtRx::Dropped(MgmtDrop::NotAssociated));
    }
    let (mut s, ptk) = joined_pmf();
    let mut bad = sealed(&mgmt(DEAUTH, SPA, &[1, 0]), 1, &ptk, false);
    let n = bad.len();
    bad[n - 1] ^= 1;
    assert_eq!(s.receive_mgmt(&bad, false), MgmtRx::Dropped(MgmtDrop::Undecryptable));
    assert!(s.is_associated());
}

#[test]
fn a_broadcast_deauthentication_needs_bip_above_the_floor() {
    let (mut s, _) = joined_pmf();
    let plain = mgmt(DEAUTH, BCAST, &[3, 0]);
    let at_floor = bip_protect(&plain, 4, 1, &IGTK).unwrap();
    assert_eq!(s.receive_mgmt(&at_floor, false), MgmtRx::Dropped(MgmtDrop::Replay));
    let other_index = bip_protect(&plain, 5, 2, &IGTK).unwrap();
    assert_eq!(s.receive_mgmt(&other_index, false), MgmtRx::Dropped(MgmtDrop::Undecryptable));
    let good = bip_protect(&plain, 4, 2, &IGTK).unwrap();
    assert_eq!(s.receive_mgmt(&good, false), MgmtRx::Left(3));
}
