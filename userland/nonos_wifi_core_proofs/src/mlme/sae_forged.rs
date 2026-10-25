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
//! A forged SAE refusal does not end a WPA3 join: the MLME stays
//! authenticating, says nothing, and keeps the reason for a driver whose
//! deadline then ends the join.

use nonos_wifi_core::mlme::{MlmeFailure, MlmeState};
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::sae::SaeFailure;

use super::frames::sae_from_ap;
use super::sim::beacon;
use super::sim::{join, AP};
use crate::ap_sim::AP_RSNE_MIXED;

#[test]
fn a_forged_sae_refusal_leaves_the_join_running_with_its_reason() {
    let mut m = join(JoinPolicy::ANY);
    assert!(m.on_mgmt(&beacon(AP, &[&AP_RSNE_MIXED])).tx.is_some(), "the commit");
    assert_eq!(m.failure(), None);
    for (seq, status) in [(1, 1u16), (2, 1), (1, 77)] {
        assert!(m.on_mgmt(&sae_from_ap(seq, status, &[])).tx.is_none());
        assert_eq!(m.state(), MlmeState::Authenticating, "seq {seq} status {status}");
    }
    assert_eq!(m.failure(), Some(MlmeFailure::Sae(SaeFailure::GroupNotSupported)));
}
