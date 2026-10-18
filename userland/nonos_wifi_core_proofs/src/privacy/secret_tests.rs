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

//! Row 1.1: a session keeps the least secret its AKM needs, in a vault
//! that is neither Copy nor Clone (the crate checks that when it builds)
//! and is wiped when dropped: the PMK for PSK, the password token for SAE
//! by hash-to-element, the password only for hunting and pecking.

use crate::soft::join::{joined, PASS, SSID};
use nonos_wifi_core::mlme::{Entropy, Mlme};
use nonos_wifi_core::sae::SaeStation;
use nonos_wifi_core::softmac::roam::Secret;
use nonos_wifi_core::station::LinkStation;
use nonos_wifi_core::vault::Vault;
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::psk::psk;
use nonos_wifi_core::wpa::supplicant::Supplicant;

#[test]
fn each_akm_keeps_only_what_it_needs() {
    let pmk = psk(PASS, SSID).unwrap();
    for akm in [Akm::Psk, Akm::PskSha256] {
        let s = Secret::new(akm, false, SSID, PASS).unwrap();
        assert!(matches!(&s, Secret::Pmk(v) if v.get() == pmk), "{akm:?}: the PMK");
        assert!(s.password().is_empty(), "and no passphrase");
    }
    let h2e = Secret::new(Akm::Sae, true, SSID, PASS).unwrap();
    assert!(matches!(&h2e, Secret::Pt(v) if v.get().len() == 64), "the password token");
    assert!(h2e.password().is_empty());
    let hnp = Secret::new(Akm::Sae, false, SSID, PASS).unwrap();
    assert_eq!(hnp.password(), PASS, "hunting and pecking needs the password");
}

#[test]
fn a_psk_session_keeps_the_pmk_not_the_passphrase() {
    let st = joined();
    let kit = st.session().unwrap().kit.as_ref().expect("a kit");
    assert!(matches!(&kit.secret, Secret::Pmk(v) if v.get() == psk(PASS, SSID).unwrap()));
    assert!(kit.secret.password().is_empty());
}

#[test]
fn every_holder_of_a_secret_wipes_it_when_dropped() {
    use core::mem::needs_drop;
    assert!(needs_drop::<Vault<32>>() && needs_drop::<Secret>());
    assert!(needs_drop::<Mlme>() && needs_drop::<Entropy>() && needs_drop::<Supplicant>());
    assert!(needs_drop::<SaeStation>() && needs_drop::<LinkStation>());
    assert!(Vault::<4>::new(b"12345").is_none(), "a vault holds no more than its size");
    assert_eq!(Vault::<8>::new(b"1234").unwrap().get(), b"1234");
}
