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

//! The frames krack.rs sends after a join: one EAPOL frame from the AP,
//! protected under the pairwise key and serviced through Port, and the
//! join's own message 3 under a chosen replay counter.

use nonos_wifi_core::wpa::akm::Akm;

use crate::ap_sim::{gtk_kde, message3, GTK};
use crate::gen3::join::fw::Fw;
use crate::gen3::join::link::Port;
use crate::gen3::join::run::Joined;
use crate::gen3_model::{Mem, Model, Polls};
use crate::join_rig::*;

pub(super) const SEC_KEY: (u8, u8) = (0x5, 0x18);

/// One EAPOL frame from the AP, protected under the pairwise key, serviced.
pub(super) fn deliver(
    r: &Rig,
    fw: &mut Fw<'_, '_, Model, Mem, Polls>,
    j: &mut Joined,
    eapol: &[u8],
) {
    let mut snap = vec![0xAA, 0xAA, 0x03, 0, 0, 0, 0x88, 0x8E];
    snap.extend_from_slice(eapol);
    let pkt = r.ap.borrow_mut().protected(&snap);
    r.model.queue(pkt);
    Port { fw, link: &mut j.link }.service();
}

pub(super) fn message3_again(r: &Rig, replay: u8) -> Vec<u8> {
    let ap = r.ap.borrow();
    let mut kd = ap.rsne.clone();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    message3(Akm::Psk, replay, &ANONCE, &ap.ptk, &kd)
}
