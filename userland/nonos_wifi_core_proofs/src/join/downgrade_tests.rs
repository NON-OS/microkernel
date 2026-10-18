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

//! End to end, through the control face a panel speaks: a network the person
//! saved as WPA3 (the join's flag bit 0) is refused as a downgrade when the
//! access point answering to its name offers only WPA2, the access point is
//! never keyed, and the passphrase never leaves on the refused join. The
//! honest WPA2 join of the same access point is the baseline.

use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::rig::{rig, Station};
use crate::soft::wire::{code, connect};
use nonos_wifi_core::softmac::control::result::CODE_DOWNGRADE;
use nonos_wifi_core::softmac::join::FLAG_WPA3_ONLY;

const SSID: &[u8] = b"ThisIsASSID";
const PASS: &[u8] = b"ThisIsAPassword";

fn wpa2_ap() -> Station {
    rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK))
}

fn leaks_pass(st: &Station) -> bool {
    let air = st.radio().unwrap();
    air.sent.iter().any(|f| f.windows(PASS.len()).any(|w| w == PASS))
}

#[test]
fn honest_wpa2_join_is_the_baseline() {
    let mut st = wpa2_ap();
    let r = connect(&mut st, SSID, PASS, 0);
    assert_eq!((code(&r), r[33]), (0, 2), "the same AP joins as WPA2-PSK");
    assert!(st.radio().unwrap().ap.tk.is_some(), "the AP is keyed on the honest join");
}

#[test]
fn saved_wpa3_refuses_a_wpa2_only_ap() {
    let mut st = wpa2_ap();
    let r = connect(&mut st, SSID, PASS, FLAG_WPA3_ONLY);
    assert_eq!(code(&r), CODE_DOWNGRADE, "saved WPA3, offered only WPA2: a downgrade");
    let air = st.radio().unwrap();
    assert!(air.ap.tk.is_none(), "the AP was never keyed");
    assert!(air.ap.ptk.is_none(), "no pairwise key was ever derived at the AP");
    assert!(air.ap.got.is_empty(), "no protected data reached the AP");
    assert!(!leaks_pass(&st), "the passphrase never left on the refused join");
}

#[test]
fn the_refusal_does_not_fall_back_to_psk() {
    // A WPA2 retry would key the AP; a downgrade must not retry.
    let mut st = wpa2_ap();
    connect(&mut st, SSID, PASS, FLAG_WPA3_ONLY);
    assert!(st.radio().unwrap().ap.tk.is_none(), "no WPA2 fallback after a downgrade");
}
