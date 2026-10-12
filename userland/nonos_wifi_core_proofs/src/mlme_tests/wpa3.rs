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

//! WPA3 to Connected, and a network saved as WPA3 that shows PSK only.

use nonos_wifi_core::eapol::mic::verify_mic_kind;
use nonos_wifi_core::mlme::{MlmeFailure, MlmeState};
use nonos_wifi_core::rsn::{JoinPolicy, SelectError};
use nonos_wifi_core::wpa::akm::Akm;

use super::base::{join, AP, SNONCE, STA};
use super::frames::{beacon, rsne};
use super::sae::sae_join;
use crate::ap_sim::{gtk_kde, igtk_kde, message1, message3, AP_RSNE_MIXED, GTK};

#[test]
fn a_wpa3_join_runs_sae_then_the_four_way_to_connected() {
    for h2e in [true, false] {
        let (mut m, pmk) = sae_join(h2e);
        assert_eq!(m.state(), MlmeState::FourWay);
        let anonce = [0xA2; 32];
        let m2 = m.on_eapol(&message1(Akm::Sae, 1, &anonce)).tx.expect("message 2");
        let ptk = Akm::Sae.derive_ptk(&pmk, &AP, &STA, &anonce, &SNONCE);
        assert!(verify_mic_kind(Akm::Sae.mic_kind(), &ptk[..16], &m2), "the SAE PMK keys it");
        let mut kd = AP_RSNE_MIXED.to_vec();
        if h2e {
            kd.extend_from_slice(&[244, 1, 0x20]);
        }
        kd.extend_from_slice(&gtk_kde(1, &GTK));
        // SAE runs with management frame protection, so the IGTK comes too.
        kd.extend_from_slice(&igtk_kde(&[0x77; 16]));
        assert!(m.on_eapol(&message3(Akm::Sae, 2, &anonce, &ptk, &kd)).tx.is_some());
        assert_eq!(m.state(), MlmeState::Connected, "h2e={h2e}");
        assert_eq!(m.akm(), Some(Akm::Sae));
    }
}

#[test]
fn a_network_saved_as_wpa3_is_refused_when_it_shows_psk_only() {
    let mut m = join(JoinPolicy::WPA3_ONLY);
    assert!(m.on_mgmt(&beacon(AP, &[&rsne(&[2], 0, 4)])).tx.is_none(), "nothing is sent");
    assert_eq!(m.failure(), Some(MlmeFailure::Select(SelectError::Downgrade)));
}
