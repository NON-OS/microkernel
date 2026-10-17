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

//! The join report carries the indication from the AP's message 3, so the
//! caller can keep it with the saved network.

use crate::mlme::sim::entropy;
use crate::soft::air::STA;
use crate::soft::ap::SimAp;
use crate::soft::env::SimEnv;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{PASS, SSID};
use crate::soft::rig::rig;
use nonos_wifi_core::eapol::kde::TD_WPA3_PERSONAL;
use nonos_wifi_core::mlme::{JoinRequest, Mlme};
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::softmac::join::{drive, Outcome, Report, JOIN_BUDGET_MS};

fn joined_with(bits: Option<u8>) -> Report {
    let mut ap = SimAp::new(SSID, PASS, 1, &RSNE_PSK);
    ap.quirks.transition_disable = bits;
    let beacon = ap.beacon();
    let mut st = rig(ap);
    let air = st.radio_mut().unwrap();
    air.tuned = Some((Band::Ghz2, 1));
    let env = SimEnv { clock: air.clock.clone(), draws: 0 };
    let policy = JoinPolicy::ANY;
    let req =
        JoinRequest { our_mac: STA, ssid: SSID, passphrase: PASS, policy, entropy: entropy() };
    drive(air, &env, Mlme::join(&req), &beacon, JOIN_BUDGET_MS, 0)
}

#[test]
fn the_join_report_carries_the_aps_bits() {
    let r = joined_with(Some(TD_WPA3_PERSONAL));
    assert!(matches!(r.outcome, Outcome::Joined { .. }));
    assert_eq!(r.transition_disable, Some(TD_WPA3_PERSONAL));
}

#[test]
fn without_the_kde_the_report_has_none() {
    let r = joined_with(None);
    assert!(matches!(r.outcome, Outcome::Joined { .. }));
    assert!(r.transition_disable.is_none());
}
