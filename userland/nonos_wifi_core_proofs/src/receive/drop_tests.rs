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

//! Everything that is not for this link is dropped with its reason: another
//! BSS, ToDS, another station, fragments, A-MSDUs, unprotected data on a
//! protected link, a forged MIC, a runt, a replay, and any frame before
//! association.

use crate::receive::ccmp_tests::tk;
use crate::receive::frame::{from_ap, station, STA};
use crate::receive::join::ip_body;
use nonos_wifi_core::station::{LinkStation, Rx, RxDrop};

#[test]
fn frames_that_are_not_for_this_link_are_dropped_with_a_reason() {
    let mut s = station();
    let good = from_ap(STA, &ip_body(b"x"), Some(&tk()), 1, 0, true);
    let mut other_bss = good.clone();
    other_bss[10..16].copy_from_slice(&[0x02, 0, 0, 0, 0, 0x55]);
    assert!(matches!(s.receive(&other_bss, false), Rx::Dropped(RxDrop::NotFromBss)));
    let mut to_ds = good.clone();
    to_ds[1] = (to_ds[1] & !0x03) | 0x01;
    assert!(matches!(s.receive(&to_ds, false), Rx::Dropped(RxDrop::NotFromBss)));
    let mut other_sta = good.clone();
    other_sta[4..10].copy_from_slice(&[0x02, 0, 0, 0, 0, 0x33]);
    assert!(matches!(s.receive(&other_sta, false), Rx::Dropped(RxDrop::NotForUs)));
    let mut fragment = good.clone();
    fragment[1] |= 0x04;
    assert!(matches!(s.receive(&fragment, false), Rx::Dropped(RxDrop::Fragmented)));
    let mut amsdu = good.clone();
    amsdu[24] |= 0x80;
    assert!(matches!(s.receive(&amsdu, false), Rx::Dropped(RxDrop::Amsdu)));
    let plain = from_ap(STA, &ip_body(b"injected"), None, 0, 0, true);
    assert!(matches!(s.receive(&plain, false), Rx::Dropped(RxDrop::Unprotected)));
    let mut tampered = good.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 1;
    assert!(matches!(s.receive(&tampered, false), Rx::Dropped(RxDrop::Undecryptable)));
    assert!(matches!(s.receive(&good[..30], false), Rx::Dropped(RxDrop::Malformed)));
    assert!(matches!(s.receive(&good, false), Rx::Ethernet(_)));
    assert!(matches!(s.receive(&good, false), Rx::Dropped(RxDrop::Replay)));
    let mut unassociated = LinkStation::new(STA);
    assert!(matches!(unassociated.receive(&good, false), Rx::Dropped(RxDrop::NotAssociated)));
}
