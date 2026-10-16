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

//! An integrity group key of all zeros is never installed either: a group
//! message 1 carrying one is unusable and goes unanswered, and the same
//! message with a real IGTK is answered.

use crate::ap_sim::{group1, gtk_kde, igtk_kde, SPA};
use crate::pmf_sim::{joined_pmf, tk};
use crate::receive::frame::from_ap;
use crate::receive::join::eapol_body;
use nonos_wifi_core::station::Rx;
use nonos_wifi_core::wpa::akm::Akm;

#[test]
fn an_igtk_of_zeros_is_never_installed() {
    let (mut s, ptk) = joined_pmf();
    let message = |rc: u8, igtk: &[u8; 16], pn: u64| {
        let kd = [gtk_kde(2, &[0x7e; 16]), igtk_kde(igtk)].concat();
        from_ap(SPA, &eapol_body(&group1(Akm::Psk, rc, &ptk, &kd)), Some(&tk(&ptk)), pn, 0, true)
    };
    let answered = |r: Rx| matches!(r, Rx::Handshake { frame: Some(_), .. });
    assert!(!answered(s.receive(&message(3, &[0; 16], 1), false)), "all zeros");
    assert!(answered(s.receive(&message(4, &[0x44; 16], 2), false)), "a real one");
}
