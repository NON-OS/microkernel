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

//! A PMKSA comes only of a handshake that finished. A join cut anywhere
//! (after the SAE commit, after SAE, after message 1, at a message 3 that
//! does not verify), as a capsule that dies or restarts there would leave
//! it, holds none to cache; the next start begins with SAE again.

use super::sim::{by_sae, ANONCE};
use crate::ap_sim::{gtk_kde, message1, message3, AP_RSNE_MIXED, GTK};
use crate::mlme::sae::sae_join;
use crate::mlme::sim::{beacon, join, AP, SNONCE, STA};
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::wpa::akm::Akm;

#[test]
fn a_handshake_cut_anywhere_leaves_no_pmksa() {
    let mut m = join(JoinPolicy::ANY);
    assert!(m.on_mgmt(&beacon(AP, &[&AP_RSNE_MIXED])).tx.is_some(), "the SAE commit");
    assert!(m.pmksa().is_none(), "after the commit");
    let (m, _) = sae_join(false);
    assert!(m.pmksa().is_none(), "after SAE, before the 4-way");
    let (mut m, _) = sae_join(false);
    m.on_eapol(&message1(Akm::Sae, 1, &ANONCE));
    assert!(m.pmksa().is_none(), "after message 1");
    let (mut m, _) = sae_join(false);
    m.on_eapol(&message1(Akm::Sae, 1, &ANONCE));
    let wrong = Akm::Sae.derive_ptk(&[0x5a; 32], &AP, &STA, &ANONCE, &SNONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    m.on_eapol(&message3(Akm::Sae, 2, &ANONCE, &wrong, &kd));
    assert!(m.pmksa().is_none(), "a message 3 that does not verify");
    assert!(by_sae().0.pmksa().is_some(), "only the whole handshake keeps one");
}
