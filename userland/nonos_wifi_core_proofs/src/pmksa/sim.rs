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

//! A join against the simulated AP up to Connected, for the PMKSA proofs:
//! one by SAE, which leaves a PMKSA to cache, and one on a cached PMKSA,
//! which runs Open System authentication instead (IEEE Std 802.11-2020,
//! 12.6.10.3).

use crate::ap_sim::{gtk_kde, igtk_kde, message1, message3, AP_RSNE_MIXED, GTK};
use crate::mlme::frames::{assoc_elements, assoc_resp, open_auth_ok};
use crate::mlme::sae::sae_join;
use crate::mlme::sim::{beacon, join, AP, SNONCE, STA};
use nonos_wifi_core::mlme::{Mlme, MlmeState};
use nonos_wifi_core::pmksa::Pmksa;
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::wpa::akm::Akm;

pub const ANONCE: [u8; 32] = [0xA2; 32];

/// The four-way handshake under `pmk`, from message 1 to Connected; the
/// station's message 2.
pub fn four_way(m: &mut Mlme, pmk: &[u8; 32]) -> Vec<u8> {
    let m2 = m.on_eapol(&message1(Akm::Sae, 1, &ANONCE)).tx.expect("message 2");
    let ptk = Akm::Sae.derive_ptk(pmk, &AP, &STA, &ANONCE, &SNONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    // SAE runs with management frame protection, so the IGTK comes too.
    kd.extend_from_slice(&igtk_kde(&[0x77; 16]));
    assert!(m.on_eapol(&message3(Akm::Sae, 2, &ANONCE, &ptk, &kd)).tx.is_some());
    assert_eq!(m.state(), MlmeState::Connected);
    m2
}

/// A full SAE join to Connected, and the PMK it ran under.
pub fn by_sae() -> (Mlme, [u8; 32]) {
    let (mut m, pmk) = sae_join(false);
    four_way(&mut m, &pmk);
    (m, pmk)
}

/// A join offered `p`, from the beacon to the association request: the
/// authentication frame sent first, and the request's RSNE body.
pub fn offered(p: Pmksa) -> (Mlme, Vec<u8>, Vec<u8>) {
    let mut m = join(JoinPolicy::ANY);
    m.offer_pmksa(p);
    let auth = m.on_mgmt(&beacon(AP, &[&AP_RSNE_MIXED])).tx.expect("authentication");
    let req = m.on_mgmt(&open_auth_ok(AP, STA)).tx.unwrap_or_default();
    let rsne = assoc_elements(&req).into_iter().find(|e| e.0 == 48).map_or(Vec::new(), |e| e.1);
    (m, auth, rsne)
}

pub fn accepted(m: &mut Mlme) {
    m.on_mgmt(&assoc_resp(0));
    assert_eq!(m.state(), MlmeState::FourWay);
}
