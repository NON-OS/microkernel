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

//! A saved-WPA3 network demands management-frame protection. A twin access
//! point answering to its name with SAE but no protection (MFPC clear) must
//! be refused end to end, never joined in the clear. The refusal is settled
//! from the beacon's RSNE, before any authentication, so no SAE handshake is
//! needed to prove it; the access point is never keyed.

use crate::soft::ap::SimAp;
use crate::soft::rig::{rig, Station};
use crate::soft::wire::{code, connect};
use nonos_wifi_core::softmac::control::result::CODE_UNSUPPORTED;
use nonos_wifi_core::softmac::join::FLAG_WPA3_ONLY;

const SSID: &[u8] = b"ThisIsASSID";
const PASS: &[u8] = b"ThisIsAPassword";
/// A beacon RSNE: CCMP-128, AKM SAE (8), capabilities 0 — no MFPC, so the
/// access point offers SAE without the protection WPA3 requires.
const RSNE_SAE_NO_PMF: [u8; 22] =
    [0x30, 0x14, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 8, 0, 0];

fn sae_no_pmf_ap() -> Station {
    rig(SimAp::new(SSID, PASS, 6, &RSNE_SAE_NO_PMF))
}

#[test]
fn saved_wpa3_refuses_sae_without_protection() {
    let mut st = sae_no_pmf_ap();
    let r = connect(&mut st, SSID, PASS, FLAG_WPA3_ONLY);
    assert_eq!(code(&r), CODE_UNSUPPORTED, "SAE without PMF is not WPA3; refused");
    let air = st.radio().unwrap();
    assert!(air.ap.tk.is_none(), "the AP was never keyed");
    assert!(air.ap.got.is_empty(), "no data reached an unprotected AP");
    let leaks = air.sent.iter().any(|f| f.windows(PASS.len()).any(|w| w == PASS));
    assert!(!leaks, "the passphrase never left on the refused join");
}
