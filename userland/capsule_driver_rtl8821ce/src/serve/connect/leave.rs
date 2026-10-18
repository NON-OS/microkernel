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

//! Leaving the network.

use nonos_wifi_core::key::KeyStore;

use crate::fw::dma::Grant;
use crate::link::{RtlKeys, RtlLink};
use crate::regs::Regs;

use super::consts::*;
use super::media::set_media_no_link;
use super::session::Session;

/// Tell the AP the station is leaving, drop the association, unbind the MAC
/// from the BSS and clear the keys it installed.
pub(in crate::serve) fn disconnect(
    link: &mut RtlLink<Regs, Grant, Grant>,
    keys: &mut RtlKeys<Regs>,
    regs: &Regs,
    session: &mut Option<Session>,
) -> i32 {
    link.send_deauth();
    link.deassociate();
    set_media_no_link(regs);
    if let Some(s) = session.take() {
        keys.remove_key(s.key_id, Some(&s.bssid));
        keys.remove_key(s.gtk_id, None);
        // And the other group slots: an AP rekey moves the group key between
        // indices, leaving the old key in its slot.
        for id in (0..GROUP_KEY_SLOTS).filter(|&id| id != s.gtk_id) {
            keys.remove_key(id, None);
        }
    }
    0
}
