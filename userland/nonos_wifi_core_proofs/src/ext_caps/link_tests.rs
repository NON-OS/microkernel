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

//! The soft-MAC station checks beacons, so its protected joins and moves
//! offer beacon protection; its unprotected ones offer nothing.

use super::mlme_tests::BIT_84;
use crate::mlme::frames::assoc_elements;
use crate::pmf_sim::AP_RSNE_MFP;
use crate::roam::two_aps::{run_until, sent, two_aps_with, BB};
use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{PASS, SSID};
use crate::soft::rig::rig;
use crate::soft::wire::{code, connect};

fn ext_caps(f: &[u8], fixed: usize) -> Option<Vec<u8>> {
    let tail = [&f[..28], &f[28 + fixed..]].concat();
    assoc_elements(&tail).into_iter().find(|e| e.0 == 127).map(|e| e.1)
}

#[test]
fn a_protected_join_and_its_move_offer_beacon_protection() {
    let mfp = |b: &mut SimAp| b.rsne = AP_RSNE_MFP.to_vec();
    let mut st = two_aps_with(mfp, mfp, [-85, -40]);
    let joined = sent(&st, 0x00);
    assert_eq!(ext_caps(&joined[0], 0), Some(BIT_84.to_vec()), "the association");
    assert!(run_until(&mut st, |s| s.session().is_some_and(|s| s.bssid == BB)));
    let moved = sent(&st, 0x20);
    assert_eq!(ext_caps(&moved[0], 6), Some(BIT_84.to_vec()), "the reassociation");
}

#[test]
fn an_unprotected_join_offers_nothing() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0);
    assert_eq!(ext_caps(&sent(&st, 0x00)[0], 0), None);
}
